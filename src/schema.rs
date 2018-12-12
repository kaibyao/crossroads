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

table! {
    xr_user_credential (xr_id) {
        xr_id -> Int8,
        xr_uuid -> Uuid,
        xr_created_at -> Timestamptz,
        xr_updated_at -> Timestamptz,
        xr_created_by -> Nullable<Int8>,
        xr_updated_by -> Nullable<Int8>,
        xr_user_id -> Int8,
        xr_password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    xr_user,
    xr_user_credential,
);
