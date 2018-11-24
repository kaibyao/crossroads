-- This file builds the xr_user table along with the initial
-- 2 user records for "Setup" and "System". "Setup" is used for creating
-- the initial base tables at setup, while System is used for all database
-- operations at the system level.

-- We have to create the table using xr_setup_table() because
-- Diesel does not allow us to create a table without a primary key,
-- and the primary key is a universally-used (read: default) field
-- in every table, which is what xr_setup_table() is used for.
SELECT xr_setup_table('xr_user');

ALTER TABLE xr_user
  ADD COLUMN xr_email text NOT NULL check (xr_email != ''),
  ADD COLUMN xr_first_name text NOT NULL check (xr_first_name != ''),
  ADD COLUMN xr_last_name text NOT NULL check (xr_last_name != ''),
  ADD COLUMN xr_user_name text NOT NULL check (xr_user_name != ''),
  ADD CONSTRAINT unique_user_name UNIQUE (xr_user_name),
  ADD CONSTRAINT unique_email UNIQUE (xr_email)
;

INSERT INTO xr_user (
  xr_email,
  xr_first_name,
  xr_last_name,
  xr_user_name
) VALUES
  (
    'setup@crossroads',
    'Setup',
    'Crossroads',
    'setup'
  ),
  (
    'system@crossroads',
    'System',
    'Crossroads',
    'system'
  );

WITH
  setup AS (
    SELECT xr_id FROM xr_user WHERE xr_user_name = 'setup'
  )
UPDATE xr_user SET
  xr_updated_by = (SELECT xr_id FROM setup),
  xr_created_by = (SELECT xr_id FROM setup);
