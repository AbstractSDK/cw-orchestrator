#[derive(Clone)]
pub enum Attempts {
    Unlimited,
    Count(usize),
}

impl std::fmt::Display for Attempts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attempts::Unlimited => write!(f, "unlimited")?,
            Attempts::Count(count) => write!(f, "{}", count)?,
        }
        Ok(())
    }
}

impl Attempts {
    pub fn can_retry(&self) -> bool {
        match self {
            Attempts::Unlimited => true,
            Attempts::Count(count) => *count > 0,
        }
    }

    /// Verifies the attempt can retry
    /// If it can retry, decrements the counter
    pub fn retry(&mut self) -> bool {
        let can_retry = self.can_retry();
        if can_retry {
            self.decrement();
        }
        can_retry
    }
    fn decrement(&mut self) {
        if let Attempts::Count(count) = self {
            *count -= 1
        }
    }
}
