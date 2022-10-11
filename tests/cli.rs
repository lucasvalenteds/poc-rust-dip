use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use testcontainers::clients::Cli;
use testcontainers::images::postgres::Postgres;

#[test]
fn getting_current_date_time_from_runtime() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // Example: 2022-10-11 03:19:04.157182449
    let timestamp_pattern =
        predicate::str::is_match("[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2}.*");

    command
        .assert()
        .stdout(predicate::str::starts_with("The current date is "))
        .stdout(timestamp_pattern.unwrap())
        .stdout(predicate::str::ends_with(" UTC\n"))
        .success();

    Ok(())
}

#[test]
fn getting_current_date_time_from_database() -> Result<(), Box<dyn std::error::Error>> {
    // Provisioning Postgres instance using Docker
    let docker_client = Cli::default();
    let docker_image = Postgres::default();
    let docker_container = docker_client.run(docker_image);

    // Running application (CLI) and asserting the output
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    command.env(
        "DATABASE_URL",
        format!(
            "postgresql://postgres:postgres@localhost:{}/postgres",
            docker_container.get_host_port_ipv4(5432)
        ),
    );

    // Example: 2022-10-11 03:19:04.157182449
    let timestamp_pattern =
        predicate::str::is_match("[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2}.*");

    command
        .assert()
        .stdout(predicate::str::starts_with("The current date is "))
        .stdout(timestamp_pattern.unwrap())
        .stdout(predicate::str::ends_with(" UTC\n"))
        .success();

    Ok(())
}
