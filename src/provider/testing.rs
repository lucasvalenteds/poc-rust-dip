use crate::provider::DateTimeProvider;
use chrono::{DateTime, Utc};

pub struct TestingDateTimeProvider {
    pub invocations: usize,
}

impl DateTimeProvider for TestingDateTimeProvider {
    fn get_current_date_time(&mut self) -> Result<DateTime<Utc>, String> {
        if self.invocations % 2 == 0 {
            self.invocations += 1;
            Ok("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>().unwrap())
        } else {
            self.invocations += 1;
            Err(String::from("Error executing query"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::provider::testing::TestingDateTimeProvider;
    use crate::provider::DateTimeProvider;

    #[test]
    fn getting_current_date_time() {
        let mut provider: Box<dyn DateTimeProvider> =
            Box::new(TestingDateTimeProvider { invocations: 0 });

        let current_date_1 = provider.get_current_date_time();
        let current_date_2 = provider.get_current_date_time();

        assert_eq!(true, current_date_1.is_ok());
        assert_eq!(true, current_date_2.is_err());
        assert_eq!(
            "2014-11-28 12:00:09 UTC",
            current_date_1.unwrap().to_string()
        );
        assert_eq!("Error executing query", current_date_2.unwrap_err());
    }
}
