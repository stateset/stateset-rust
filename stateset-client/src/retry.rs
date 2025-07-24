//! Retry policy implementation for StateSet SDK

use std::time::Duration;

/// Retry policy for HTTP requests
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
    pub jitter: bool,
}

impl RetryPolicy {
    /// Create a new retry policy
    pub fn new(
        max_attempts: u32,
        initial_delay: Duration,
        max_delay: Duration,
        multiplier: f64,
    ) -> Self {
        Self {
            max_attempts,
            initial_delay,
            max_delay,
            multiplier,
            jitter: true,
        }
    }

    /// Create a retry policy with no jitter
    pub fn without_jitter(mut self) -> Self {
        self.jitter = false;
        self
    }

    /// Calculate the delay for a given attempt number
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return self.initial_delay;
        }

        let base_delay = self.initial_delay.as_millis() as f64 * self.multiplier.powi(attempt as i32);
        let delay = Duration::from_millis(base_delay as u64);
        let capped_delay = std::cmp::min(delay, self.max_delay);

        if self.jitter {
            self.add_jitter(capped_delay)
        } else {
            capped_delay
        }
    }

    /// Add jitter to prevent thundering herd problems
    fn add_jitter(&self, delay: Duration) -> Duration {
        use rand::Rng;
        
        let jitter_factor = rand::thread_rng().gen_range(0.5..=1.5);
        let jittered_ms = (delay.as_millis() as f64 * jitter_factor) as u64;
        
        Duration::from_millis(jittered_ms)
    }

    /// Check if we should retry for the given attempt
    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt < self.max_attempts
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(
            3,
            Duration::from_millis(1000),
            Duration::from_secs(60),
            2.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_creation() {
        let policy = RetryPolicy::new(
            3,
            Duration::from_millis(500),
            Duration::from_secs(30),
            2.0,
        );

        assert_eq!(policy.max_attempts, 3);
        assert_eq!(policy.initial_delay, Duration::from_millis(500));
        assert_eq!(policy.max_delay, Duration::from_secs(30));
        assert_eq!(policy.multiplier, 2.0);
    }

    #[test]
    fn test_delay_calculation() {
        let policy = RetryPolicy::new(
            3,
            Duration::from_millis(1000),
            Duration::from_secs(60),
            2.0,
        ).without_jitter();

        assert_eq!(policy.delay_for_attempt(0), Duration::from_millis(1000));
        assert_eq!(policy.delay_for_attempt(1), Duration::from_millis(2000));
        assert_eq!(policy.delay_for_attempt(2), Duration::from_millis(4000));
    }

    #[test]
    fn test_delay_capping() {
        let policy = RetryPolicy::new(
            5,
            Duration::from_millis(1000),
            Duration::from_millis(3000),
            2.0,
        ).without_jitter();

        assert_eq!(policy.delay_for_attempt(0), Duration::from_millis(1000));
        assert_eq!(policy.delay_for_attempt(1), Duration::from_millis(2000));
        assert_eq!(policy.delay_for_attempt(2), Duration::from_millis(3000)); // Capped
        assert_eq!(policy.delay_for_attempt(3), Duration::from_millis(3000)); // Still capped
    }

    #[test]
    fn test_should_retry() {
        let policy = RetryPolicy::new(
            3,
            Duration::from_millis(1000),
            Duration::from_secs(60),
            2.0,
        );

        assert!(policy.should_retry(0));
        assert!(policy.should_retry(1));
        assert!(policy.should_retry(2));
        assert!(!policy.should_retry(3));
        assert!(!policy.should_retry(4));
    }

    #[test]
    fn test_jitter_adds_randomness() {
        let policy = RetryPolicy::new(
            3,
            Duration::from_millis(1000),
            Duration::from_secs(60),
            2.0,
        );

        let delay1 = policy.delay_for_attempt(1);
        let delay2 = policy.delay_for_attempt(1);
        
        // With jitter, delays should be different (most of the time)
        // Note: This test has a small chance of flaking if random values are the same
        // but it's very unlikely with the jitter range we use
        assert_ne!(delay1, delay2);
    }
}