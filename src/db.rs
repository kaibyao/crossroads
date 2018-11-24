use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde_derive::{Deserialize, Serialize};
use std::process::Command;

/// Describes the DB config used to connect with the database.
#[derive(Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
}

/// Establishes and returns an active PostgreSQL connection
pub fn establish_connection(config: &DatabaseConfig) -> PgConnection {
    let database_url = create_postgres_url(config);

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// Creates a PostgreSQL URL in the format of postgresql://[user[:password]@][netloc][:port][/dbname]
pub fn create_postgres_url(config: &DatabaseConfig) -> String {
    let mut database_url = String::from("postgresql://");

    if config.db_user != "" {
        database_url.push_str(&config.db_user);

        if config.db_pass != "" {
            database_url.push_str(&format!(":{}", &config.db_pass))
        }

        database_url.push_str("@");
    }

    database_url.push_str(&format!(
        "{}:{}/{}",
        &config.db_host, &config.db_port, &config.db_name
    ));

    database_url
}

/// A wrapper around dropping the database (diesel database reset).
pub fn drop_database(config: &DatabaseConfig) {
    Command::new("diesel")
        .env("DATABASE_URL", &create_postgres_url(config))
        .arg("database")
        .arg("reset")
        .output()
        .unwrap();
}

/// Checks that the given identifier (table name, column name, etc) is valid in Postgres. Useful as a defense against SQL injection.
/// Returns true if valid and false otherwise.
///
/// # Example:
/// ```
/// use crossroads::db::is_valid_postgres_name;
/// assert!(!is_valid_postgres_name("1_column"));
/// assert!(is_valid_postgres_name("_column_1"));
/// assert!(is_valid_postgres_name("column_1"));
/// assert!(!is_valid_postgres_name("column_1_$"));
/// assert!(!is_valid_postgres_name("*olumn_1"));
/// assert!(!is_valid_postgres_name("column_1; DROP DATABASE;"));
/// ```
pub fn is_valid_postgres_name(name: &str) -> bool {
    use regex::Regex;

    // names must begin with either a letter or an underscore and only contain letters, numbers, and underscores
    Regex::new(r"(?i)^[a-z_]{1}([a-z0-9_]*)$")
        .unwrap()
        .is_match(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_postgres_url_correct() {
        let config = DatabaseConfig {
            db_host: "test_host".to_string(),
            db_port: 1234,
            db_user: "test_user".to_string(),
            db_pass: "test_pass".to_string(),
            db_name: "test_db".to_string(),
        };

        assert_eq!(
            create_postgres_url(&config),
            "postgresql://test_user:test_pass@test_host:1234/test_db"
        );
    }
}
