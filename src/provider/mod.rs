pub mod database;
pub mod runtime;
pub mod testing;

use chrono::{DateTime, Utc};

pub trait DateTimeProvider {
    fn get_current_date_time(&mut self) -> Result<DateTime<Utc>, String>;
}
