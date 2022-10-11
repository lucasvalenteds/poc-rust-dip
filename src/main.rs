mod provider;

use crate::provider::database::DatabaseDateTimeProvider;
use crate::provider::runtime::RuntimeDateTimeProvider;
use crate::provider::DateTimeProvider;
use postgres::{Client, NoTls};
use std::env;

fn print_current_date_time(mut date_time_provider: Box<dyn DateTimeProvider>) {
    let timestamp = date_time_provider
        .get_current_date_time()
        .map(|timestamp| format!("The current date is {}", timestamp))
        .unwrap_or_else(|_| "Error getting current date".to_string());

    println!("{}", timestamp)
}

fn main() {
    let database_url: Option<String> = env::var("DATABASE_URL")
        .map(|string| Some(string))
        .unwrap_or_else(|_| None);

    let date_time_provider: Box<dyn DateTimeProvider> = match database_url {
        Some(database_url) => Box::new(DatabaseDateTimeProvider {
            client: Client::connect(database_url.as_str(), NoTls)
                .map_err(|error| format!("Error connecting to the database: {}", error))
                .unwrap(),
        }),
        None => Box::new(RuntimeDateTimeProvider {}),
    };

    print_current_date_time(date_time_provider);
}
