use embassy_time::Timer;
use futures::future::BoxFuture;

pub type Duration = embassy_time::Duration;
pub type Instant = embassy_time::Instant;
pub type Sleep = Timer;

pub fn sleep(duration: Duration) -> Sleep {
    Timer::after(duration)
}

pub fn sleep_until(deadline: Instant) -> Sleep {
    Timer::at(deadline)
}
