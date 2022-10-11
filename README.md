# POC: Dependency Inversion Principle (DIP)

It demonstrates the [Dependency Inversion Principle](https://en.wikipedia.org/wiki/Dependency_inversion_principle) (DIP)
using Rust's trait feature.

The goal is to develop a command-line application that prints the current date and time when executed. We want to define
a trait that defines the behavior of obtaining the current timestamp and write a simple function that depends on any
implementation of that trait instead of a specific one.

We provide three base implementations to demonstrate the concept: database, runtime and testing. The database
implementation connects to a Postgres database and executes a SQL query that returns the current timestamp. The runtime
implementation gets the current timestamp from the system clock based on UTC. The testing implementation purpose is to
demonstrate that we can also have stubs to use in unit tests from other components that depends on the trait.

The application checks the environment for a database URL and selects the appropriate date time provider based on that.
All the implementations as well as the function that depends on the trait should be tested using automated tests with a
real database provisioned automatically using TestContainers.

## How to run

| Description                         | Command                        |
|:------------------------------------|:-------------------------------|
| Run tests                           | `cargo test`                   |
| Print date time from system clock   | `cargo run`                    |
| Print date time from database query | `DATABASE_URL=<url> cargo run` |
| Provision the database              | `docker-compose up --detach`   |
| Destroy the database                | `docker-compose down`          |

## Preview

Printing the current date time obtained from system clock:

```
$ time cargo run
The current date is 2022-10-11 23:11:00.326328794 UTC

real	0m0,074s
user	0m0,057s
sys	0m0,017s
```

Printing the current date time obtained from database:

```
$ time DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres cargo run
The current date is 2022-10-11 23:10:53.578463 UTC

real	0m0,162s
user	0m0,146s
sys	0m0,013s
```

