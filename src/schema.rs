#![allow(proc_macro_derive_resolution_fallback)]

table! {
    xr_user (xr_id) {
        xr_id -> Int8,
        xr_uuid -> Uuid,
        xr_created_at -> Timestamptz,
        xr_updated_at -> Timestamptz,
        xr_created_by -> Nullable<Int8>,
        xr_updated_by -> Nullable<Int8>,
        xr_email -> Text,
        xr_first_name -> Text,
        xr_last_name -> Text,
        xr_user_name -> Text,
    }
}
