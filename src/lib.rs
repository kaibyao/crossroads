#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate regex;
extern crate serde_derive;
extern crate uuid;

use db::{create_postgres_url, drop_database, establish_connection, DatabaseConfig};

use diesel::prelude::*;
use std::process::Command;

diesel_migrations::embed_migrations!("./migrations");

pub mod db;
pub mod models;
pub mod schema;

pub struct GenerateConfig<'a> {
    pub is_drop_db: bool,
    pub admin_email: &'a str,
    pub admin_first_name: &'a str,
    pub admin_last_name: &'a str,
    pub admin_user_name: &'a str,
}

/// Generates the initial tables used in a Crossroads application.
pub fn generate_tables(db_config: &DatabaseConfig, gen_config: &GenerateConfig) {
    let postgres_url = create_postgres_url(db_config);

    if gen_config.is_drop_db {
        drop_database(db_config);
    }

    // run diesel setup
    Command::new("diesel")
        .env("DATABASE_URL", &postgres_url)
        .arg("setup")
        .output()
        .unwrap();

    // use diesel to run all the migration SQL files
    let conn = establish_connection(db_config);
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("ERROR: Could not run Crossroads database migration. Exiting...");

    let admin_insert_row = insert_admin_user(conn, gen_config);
    assert_eq!(1, admin_insert_row);
}

// get admin details from gen_config and insert admin user into xr_user table
fn insert_admin_user(conn: diesel::PgConnection, gen_config: &GenerateConfig) -> usize {
    use diesel::insert_into;
    use models::XrUser;
    use schema::xr_user::dsl::*;

    // get id of user "system"
    let setup_user_query = xr_user
        .filter(xr_user_name.eq("setup"))
        .load::<XrUser>(&conn)
        .expect("Error fetching xr_user `setup`. Please make sure that the connection to the database is established.");
    let setup_user = &setup_user_query[0];

    // insert admin user into xr_user table
    let rows_inserted = match insert_into(xr_user).values((
        xr_email.eq(gen_config.admin_email),
        xr_first_name.eq(gen_config.admin_first_name),
        xr_last_name.eq(gen_config.admin_last_name),
        xr_user_name.eq(gen_config.admin_user_name),
        xr_created_by.eq(setup_user.xr_id),
        xr_updated_by.eq(setup_user.xr_id),
    )).execute(&conn) {
        Ok(inserted) => inserted,
        Err(e) => panic!(e)
    };

    rows_inserted
}
