use crate::provider::DateTimeProvider;

use chrono::{DateTime, Utc};
use postgres::{Client, Row};

pub struct DatabaseDateTimeProvider {
    pub client: Client,
}

impl DateTimeProvider for DatabaseDateTimeProvider {
    fn get_current_date_time(&mut self) -> Result<DateTime<Utc>, String> {
        let row: Row = self
            .client
            .query_one("SELECT NOW()", &[])
            .map_err(|error| format!("Error executing query: {}", error))?;

        let timestamp: DateTime<Utc> = row
            .try_get(0)
            .map_err(|error| format!("Error reading query row: {}", error))?;

        Ok(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use crate::provider::database::DatabaseDateTimeProvider;
    use crate::provider::DateTimeProvider;
    use postgres::{Client, NoTls};
    use std::thread;
    use std::time::Duration;
    use testcontainers::clients::Cli;
    use testcontainers::images::postgres::Postgres;

    #[test]
    fn getting_current_date_time() {
        // Provisioning Postgres instance using Docker
        let docker_client = Cli::default();
        let docker_image = Postgres::default();
        let docker_container = docker_client.run(docker_image);

        // Connecting to the database provisioned
        let database_url = &format!(
            "postgresql://postgres:postgres@localhost:{}/postgres",
            docker_container.get_host_port_ipv4(5432)
        );
        let database_tls_mode = NoTls;
        let database_client = Client::connect(database_url, database_tls_mode).unwrap();

        // Testing the implementation
        let mut provider: Box<dyn DateTimeProvider> = Box::new(DatabaseDateTimeProvider {
            client: database_client,
        });

        let current_date_1 = provider.get_current_date_time();
        thread::sleep(Duration::from_secs(1));
        let current_date_2 = provider.get_current_date_time();

        assert_eq!(true, current_date_1.is_ok());
        assert_eq!(true, current_date_2.is_ok());
        assert_ne!(current_date_1.unwrap(), current_date_2.unwrap());
    }
}
