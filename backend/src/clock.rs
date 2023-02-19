use async_std::sync::Mutex;
use chrono::prelude::*;
use lazy_static::lazy_static;

#[cfg(test)]
use std::future::Future;

#[cfg(test)]
pub async fn freeze_time<T, F, Fut>(time: DateTime<Utc>, mut f: F) -> Fut::Output
where
    F: FnMut() -> Fut,
    Fut: Future,
{
    *FROZEN_TIME.lock().await = Some(time);
    let result = f().await;
    *FROZEN_TIME.lock().await = None;
    result
}

lazy_static! {
    static ref FROZEN_TIME: Mutex<Option<DateTime<Utc>>> = Mutex::new(None);
}

#[cfg(not(test))]
pub async fn current_time() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
pub async fn current_time() -> DateTime<Utc> {
    if let Some(time) = *FROZEN_TIME.lock().await {
        time
    } else {
        Utc::now()
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[async_std::test]
    async fn freezing_time() {
        let time = Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap();

        freeze_time::<(), _, _>(time, || async {
            assert_eq!(current_time().await, time);
        })
        .await;

        assert_ne!(current_time().await, time);
    }
}
