#![allow(proc_macro_derive_resolution_fallback)]

use chrono::{DateTime, Utc};
use schema::xr_user;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[table_name="xr_user"]
pub struct XrUser {
    pub xr_id: i64,
    pub xr_uuid: Uuid,
    pub xr_created_at: DateTime<Utc>,
    pub xr_updated_at: DateTime<Utc>,
    pub xr_created_by: Option<i64>,
    pub xr_updated_by: Option<i64>,
    pub xr_email: String,
    pub xr_first_name: String,
    pub xr_last_name: String,
    pub xr_user_name: String
}
