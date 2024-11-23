pub type Duration = tokio::time::Duration;
pub type Instant = tokio::time::Instant;
pub type Sleep = futures::future::BoxFuture<'static, ()>;

pub fn sleep(duration: Duration) -> Sleep {
    Box::pin(tokio::time::sleep(duration))
}

pub fn sleep_until(deadline: Instant) -> Sleep {
    Box::pin(tokio::time::sleep_until(deadline))
}
