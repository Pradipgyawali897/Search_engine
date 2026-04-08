use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio::time::{MissedTickBehavior, interval as tokio_interval};

pub fn spawn_refresh_job(
    service: Arc<crate::service::AppService>,
    interval: Duration,
) -> JoinHandle<()> {
    spawn_refresh_job_with(service, interval, |service| async move {
        service.refresh_content().await.map(|_| ())
    })
}

pub fn spawn_refresh_job_with<T, F, Fut, E>(
    service: Arc<T>,
    interval: Duration,
    mut refresh: F,
) -> JoinHandle<()>
where
    T: Send + Sync + 'static,
    F: FnMut(Arc<T>) -> Fut + Send + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    E: std::fmt::Display + Send + 'static,
{
    tokio::spawn(async move {
        let interval = normalize_interval(interval);
        let mut ticker = tokio_interval(interval);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            ticker.tick().await;

            if let Err(error) = refresh(Arc::clone(&service)).await {
                eprintln!("[ab_web] background refresh failed: {error}");
            }
        }
    })
}

fn normalize_interval(interval: Duration) -> Duration {
    if interval.is_zero() {
        Duration::from_secs(60)
    } else {
        interval
    }
}
