use crate::*;

pub enum Deadline {
    After(Duration),
    At(Instant),
}

impl Deadline {
    pub fn at(at: Instant) -> Self {
        Self::At(at)
    }

    pub fn after(after: Duration) -> Self {
        Self::After(after)
    }

    pub fn to_sleep(&self) -> Sleep {
        match self {
            Deadline::After(after) => sleep(*after),
            Deadline::At(at) => sleep_until(*at),
        }
    }
}

impl From<Duration> for Deadline {
    fn from(value: Duration) -> Self {
        Self::After(value)
    }
}

impl From<Instant> for Deadline {
    fn from(value: Instant) -> Self {
        Self::At(value)
    }
}
