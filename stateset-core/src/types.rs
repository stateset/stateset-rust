//! Core types for StateSet SDK

use serde::{Deserialize, Serialize, Deserializer, Serializer};
use std::fmt;
use std::str::FromStr;

/// Reference type for inventory operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceType {
    /// Product reference
    Product,
    /// Variant reference
    Variant,
    /// SKU reference
    Sku,
    /// Barcode reference
    Barcode,
    /// Custom reference
    Custom(String),
}

impl Default for ReferenceType {
    fn default() -> Self {
        Self::Product
    }
}

impl fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Product => write!(f, "product"),
            Self::Variant => write!(f, "variant"),
            Self::Sku => write!(f, "sku"),
            Self::Barcode => write!(f, "barcode"),
            Self::Custom(value) => write!(f, "{}", value),
        }
    }
}

/// Resource identifier type with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceId(String);

impl ResourceId {
    /// Create a new random resource ID
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    /// Create a resource ID from a string with validation
    pub fn from_string(id: impl Into<String>) -> Result<Self, crate::Error> {
        let id_string = id.into();
        
        // Basic validation
        if id_string.is_empty() {
            return Err(crate::Error::validation("Resource ID cannot be empty"));
        }
        
        if id_string.len() > 255 {
            return Err(crate::Error::validation("Resource ID cannot exceed 255 characters"));
        }

        // Check for valid characters (alphanumeric, hyphens, underscores)
        if !id_string.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(crate::Error::validation(
                "Resource ID can only contain alphanumeric characters, hyphens, and underscores"
            ));
        }

        Ok(Self(id_string))
    }

    /// Get the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this is a valid UUID
    pub fn is_uuid(&self) -> bool {
        uuid::Uuid::parse_str(&self.0).is_ok()
    }
}

impl Default for ResourceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ResourceId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}

impl From<uuid::Uuid> for ResourceId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid.to_string())
    }
}

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        // Note: This bypasses validation for convenience
        // Use from_string() for validated creation
        Self(value)
    }
}

impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Serialize for ResourceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ResourceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s)) // Note: This doesn't validate during deserialization
    }
}

/// Timestamp type with enhanced functionality
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(#[serde(with = "chrono::serde::ts_seconds")] pub chrono::DateTime<chrono::Utc>);

impl Timestamp {
    /// Create a new timestamp with the current time
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }

    /// Create a timestamp from a Unix timestamp
    pub fn from_unix(timestamp: i64) -> Result<Self, crate::Error> {
        chrono::DateTime::from_timestamp(timestamp, 0)
            .map(Self)
            .ok_or_else(|| crate::Error::validation("Invalid Unix timestamp"))
    }

    /// Create a timestamp from an RFC3339 string
    pub fn from_rfc3339(s: &str) -> Result<Self, crate::Error> {
        chrono::DateTime::parse_from_rfc3339(s)
            .map(|dt| Self(dt.with_timezone(&chrono::Utc)))
            .map_err(|e| crate::Error::validation(format!("Invalid RFC3339 timestamp: {}", e)))
    }

    /// Get the Unix timestamp
    pub fn unix(&self) -> i64 {
        self.0.timestamp()
    }

    /// Get the RFC3339 string representation
    pub fn rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    /// Check if this timestamp is in the past
    pub fn is_past(&self) -> bool {
        self.0 < chrono::Utc::now()
    }

    /// Check if this timestamp is in the future
    pub fn is_future(&self) -> bool {
        self.0 > chrono::Utc::now()
    }

    /// Get the duration since this timestamp
    pub fn elapsed(&self) -> chrono::Duration {
        chrono::Utc::now() - self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Self(dt)
    }
}

impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

/// Money type with currency support and arithmetic
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// Amount in the smallest currency unit (e.g., cents for USD)
    pub amount: i64,
    /// ISO 4217 currency code
    pub currency: String,
}

impl Money {
    /// Create a new money value
    pub fn new(amount: i64, currency: impl Into<String>) -> Self {
        Self {
            amount,
            currency: currency.into().to_uppercase(),
        }
    }

    /// Create money from a decimal amount (e.g., 12.34 -> 1234 cents)
    pub fn from_decimal(amount: f64, currency: impl Into<String>) -> Result<Self, crate::Error> {
        let currency_str = currency.into().to_uppercase();
        let decimal_places = currency_decimal_places(&currency_str);
        let multiplier = 10_i64.pow(decimal_places as u32);
        let amount_int = (amount * multiplier as f64).round() as i64;

        Ok(Self {
            amount: amount_int,
            currency: currency_str,
        })
    }

    /// Get the decimal representation of the amount
    pub fn to_decimal(&self) -> f64 {
        let decimal_places = currency_decimal_places(&self.currency);
        let divisor = 10_f64.powi(decimal_places as i32);
        self.amount as f64 / divisor
    }

    /// Format the money as a string with currency symbol
    pub fn format(&self) -> String {
        let decimal = self.to_decimal();
        format!("{:.2} {}", decimal, self.currency)
    }

    /// Check if this money is zero
    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }

    /// Check if this money is positive
    pub fn is_positive(&self) -> bool {
        self.amount > 0
    }

    /// Check if this money is negative
    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    /// Add two money values (must have same currency)
    pub fn add(&self, other: &Money) -> Result<Money, crate::Error> {
        if self.currency != other.currency {
            return Err(crate::Error::validation(
                format!("Cannot add {} and {}", self.currency, other.currency)
            ));
        }

        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }

    /// Subtract two money values (must have same currency)
    pub fn subtract(&self, other: &Money) -> Result<Money, crate::Error> {
        if self.currency != other.currency {
            return Err(crate::Error::validation(
                format!("Cannot subtract {} and {}", self.currency, other.currency)
            ));
        }

        Ok(Money {
            amount: self.amount - other.amount,
            currency: self.currency.clone(),
        })
    }

    /// Multiply money by a scalar
    pub fn multiply(&self, factor: f64) -> Money {
        Money {
            amount: (self.amount as f64 * factor).round() as i64,
            currency: self.currency.clone(),
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Get the number of decimal places for a currency
fn currency_decimal_places(currency: &str) -> u8 {
    match currency {
        "JPY" | "KRW" | "CLP" | "ISK" | "PYG" | "VND" | "XAF" | "XOF" | "XPF" => 0,
        "BHD" | "IQD" | "JOD" | "KWD" | "LYD" | "OMR" | "TND" => 3,
        _ => 2, // Default to 2 decimal places
    }
}

/// Address type with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: String, // ISO 3166-1 alpha-2 code
}

impl Address {
    /// Create a new address with validation
    pub fn new(
        line1: impl Into<String>,
        city: impl Into<String>,
        country: impl Into<String>,
    ) -> Result<Self, crate::Error> {
        let line1_str = line1.into();
        let city_str = city.into();
        let country_str = country.into();

        if line1_str.trim().is_empty() {
            return Err(crate::Error::validation("Address line 1 cannot be empty"));
        }

        if city_str.trim().is_empty() {
            return Err(crate::Error::validation("City cannot be empty"));
        }

        if country_str.len() != 2 {
            return Err(crate::Error::validation("Country must be a 2-letter ISO code"));
        }

        Ok(Self {
            line1: line1_str,
            line2: None,
            city: city_str,
            state: None,
            postal_code: None,
            country: country_str.to_uppercase(),
        })
    }

    /// Set the second address line
    pub fn with_line2(mut self, line2: impl Into<String>) -> Self {
        self.line2 = Some(line2.into());
        self
    }

    /// Set the state/province
    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Set the postal code
    pub fn with_postal_code(mut self, postal_code: impl Into<String>) -> Self {
        self.postal_code = Some(postal_code.into());
        self
    }

    /// Format the address as a multi-line string
    pub fn format(&self) -> String {
        let mut lines = vec![self.line1.clone()];
        
        if let Some(line2) = &self.line2 {
            if !line2.trim().is_empty() {
                lines.push(line2.clone());
            }
        }

        let mut city_line = self.city.clone();
        if let Some(state) = &self.state {
            city_line.push_str(&format!(", {}", state));
        }
        if let Some(postal) = &self.postal_code {
            city_line.push_str(&format!(" {}", postal));
        }
        lines.push(city_line);
        lines.push(self.country.clone());

        lines.join("\n")
    }
}

/// Contact information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contact {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl Contact {
    /// Create a new empty contact
    pub fn new() -> Self {
        Self {
            name: None,
            email: None,
            phone: None,
        }
    }

    /// Set the contact name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the email address with validation
    pub fn with_email(mut self, email: impl Into<String>) -> Result<Self, crate::Error> {
        let email_str = email.into();
        
        // Basic email validation
        if !email_str.contains('@') || !email_str.contains('.') {
            return Err(crate::Error::validation("Invalid email address format"));
        }

        self.email = Some(email_str);
        Ok(self)
    }

    /// Set the phone number
    pub fn with_phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }
}

impl Default for Contact {
    fn default() -> Self {
        Self::new()
    }
}

/// Expandable field that can contain either an ID or the full object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expandable<T> {
    Id(ResourceId),
    Object(T),
}

impl<T> Expandable<T> {
    /// Check if this is expanded (contains the full object)
    pub fn is_expanded(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Get the ID, whether this is expanded or not
    pub fn id(&self) -> Option<&ResourceId>
    where
        T: crate::traits::Identifiable<Id = ResourceId>,
    {
        match self {
            Self::Id(id) => Some(id),
            Self::Object(obj) => Some(obj.id()),
        }
    }

    /// Get the object if this is expanded
    pub fn object(&self) -> Option<&T> {
        match self {
            Self::Id(_) => None,
            Self::Object(obj) => Some(obj),
        }
    }

    /// Convert to the object, returning an error if not expanded
    pub fn into_object(self) -> Result<T, crate::Error> {
        match self {
            Self::Id(_) => Err(crate::Error::validation("Field is not expanded")),
            Self::Object(obj) => Ok(obj),
        }
    }
}

/// Metadata type for storing arbitrary key-value pairs
pub type Metadata = std::collections::HashMap<String, serde_json::Value>;

/// List response wrapper for API endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    /// The list of items
    pub data: Vec<T>,
    /// Whether there are more items available
    pub has_more: bool,
    /// Total count of items (if available)
    pub total_count: Option<usize>,
    /// Next page cursor (if available)
    pub next_page: Option<String>,
}

/// Create a metadata map from key-value pairs
#[macro_export]
macro_rules! metadata {
    {} => {
        std::collections::HashMap::new()
    };
    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), serde_json::json!($value));
            )+
            map
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_id_validation() {
        assert!(ResourceId::from_string("valid-id_123").is_ok());
        assert!(ResourceId::from_string("").is_err());
        assert!(ResourceId::from_string("invalid@id").is_err());
        
        let id = ResourceId::new();
        assert!(id.is_uuid());
    }

    #[test]
    fn test_money_operations() {
        let money1 = Money::new(1000, "USD"); // $10.00
        let money2 = Money::new(500, "USD");  // $5.00

        assert_eq!(money1.to_decimal(), 10.0);
        assert_eq!(money2.to_decimal(), 5.0);

        let sum = money1.add(&money2).unwrap();
        assert_eq!(sum.amount, 1500);

        let diff = money1.subtract(&money2).unwrap();
        assert_eq!(diff.amount, 500);

        let doubled = money1.multiply(2.0);
        assert_eq!(doubled.amount, 2000);
    }

    #[test]
    fn test_money_from_decimal() {
        let money = Money::from_decimal(12.34, "USD").unwrap();
        assert_eq!(money.amount, 1234);
        assert_eq!(money.to_decimal(), 12.34);
    }

    #[test]
    fn test_address_creation() {
        let address = Address::new("123 Main St", "Anytown", "US")
            .unwrap()
            .with_state("CA")
            .with_postal_code("12345");

        assert_eq!(address.line1, "123 Main St");
        assert_eq!(address.city, "Anytown");
        assert_eq!(address.country, "US");
        assert_eq!(address.state, Some("CA".to_string()));
        assert_eq!(address.postal_code, Some("12345".to_string()));
    }

    #[test]
    fn test_contact_creation() {
        let contact = Contact::new()
            .with_name("John Doe")
            .with_email("john@example.com")
            .unwrap()
            .with_phone("+1-555-123-4567");

        assert_eq!(contact.name, Some("John Doe".to_string()));
        assert_eq!(contact.email, Some("john@example.com".to_string()));
        assert_eq!(contact.phone, Some("+1-555-123-4567".to_string()));
    }

    #[test]
    fn test_metadata_macro() {
        let meta = metadata! {
            "key1" => "value1",
            "key2" => 42,
            "key3" => true,
        };

        assert_eq!(meta.len(), 3);
        assert_eq!(meta["key1"], serde_json::json!("value1"));
        assert_eq!(meta["key2"], serde_json::json!(42));
        assert_eq!(meta["key3"], serde_json::json!(true));
    }

    #[test]
    fn test_timestamp_operations() {
        let ts = Timestamp::now();
        assert!(ts.is_past() || ts.elapsed().num_milliseconds() < 100);
        
        let unix_ts = Timestamp::from_unix(1609459200).unwrap(); // 2021-01-01
        assert!(unix_ts.is_past());
        assert_eq!(unix_ts.unix(), 1609459200);
    }
} 