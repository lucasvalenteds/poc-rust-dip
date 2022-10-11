use crate::provider::DateTimeProvider;
use chrono::{DateTime, Utc};

pub struct RuntimeDateTimeProvider {}

impl DateTimeProvider for RuntimeDateTimeProvider {
    fn get_current_date_time(&mut self) -> Result<DateTime<Utc>, String> {
        Ok(Utc::now())
    }
}

#[cfg(test)]
mod tests {
    use crate::{DateTimeProvider, RuntimeDateTimeProvider};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn getting_current_date_time() {
        let mut provider: Box<dyn DateTimeProvider> = Box::new(RuntimeDateTimeProvider {});

        let current_date_1 = provider.get_current_date_time();
        thread::sleep(Duration::from_secs(1));
        let current_date_2 = provider.get_current_date_time();

        assert_eq!(true, current_date_1.is_ok());
        assert_eq!(true, current_date_2.is_ok());
        assert_ne!(current_date_1.unwrap(), current_date_2.unwrap());
    }
}
