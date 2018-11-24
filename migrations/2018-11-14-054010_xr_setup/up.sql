-- This file contains all of the database setup functions that are needed for table operations in Crossroads.




-- the pgcrypto extension is used for generating:
-- * secure password hashes
-- * UUIDs
CREATE EXTENSION pgcrypto;




-- Sets up a function that is used to generate Snowflake Ids (number-based ids that include the current shard id as well as the datetimestamp) for a number field.
--
-- # Example
--
-- ```sql
-- CREATE TABLE user (id BIGINT NOT NULL DEFAULT xr_next_id() PRIMARY KEY);
-- ```
CREATE SEQUENCE xr_next_id_seq;

CREATE OR REPLACE FUNCTION xr_next_id(OUT result bigint) AS $$
DECLARE
  our_epoch bigint := 1542173886283;  -- <-Set this to your epoch
  seq_id bigint;
  now_millis bigint;
  shard_id int := 1;  -- Set int value here per shard
  BEGIN
    SELECT nextval('xr_next_id_seq') % 1024 INTO seq_id;

    SELECT FLOOR(EXTRACT(EPOCH FROM clock_timestamp()) * 1000) INTO now_millis;
    result := (now_millis - our_epoch) << 23;
    result := result | (shard_id << 10);
    result := result | (seq_id);
  END;
$$ LANGUAGE PLPGSQL;




-- Sets up a function that can be called to create default columns + indexes for the given table.
-- Also includes the following original diesel setup function:
-- Sets up a trigger for the given table to automatically set a column called
-- `xr_updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- NOTE: You should not use Diesel’s diesel_manage_updated_at() function.
--
-- # Example
--
-- ```sql
-- SELECT xr_setup_table('user');
--
-- ALTER TABLE user
-- ADD COLUMN first_name text,
-- ADD COLUMN last_name text;
-- ```
CREATE OR REPLACE FUNCTION xr_setup_table(_tbl varchar) RETURNS VOID AS $$
BEGIN
  EXECUTE format('
    CREATE TABLE %s (
      xr_id BIGINT NOT NULL DEFAULT xr_next_id() PRIMARY KEY,
      xr_uuid UUID NOT NULL DEFAULT gen_random_uuid(),
      xr_created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
      xr_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
      xr_created_by BIGINT REFERENCES xr_user,
      xr_updated_by BIGINT REFERENCES xr_user
    );
  ', _tbl);

  EXECUTE format('
    CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON %s
    FOR EACH ROW EXECUTE PROCEDURE xr_set_updated_at();
  ', _tbl);

  EXECUTE format('
    CREATE INDEX ON %s USING hash (xr_uuid);
  ', _tbl);

  EXECUTE format('
    CREATE INDEX ON %s (xr_created_at);
  ', _tbl);

  EXECUTE format('
    CREATE INDEX ON %s (xr_updated_at);
  ', _tbl);

  EXECUTE format('
    CREATE INDEX ON %s (xr_created_by);
  ', _tbl);

  EXECUTE format('
    CREATE INDEX ON %s (xr_updated_by);
  ', _tbl);
END;
$$ LANGUAGE plpgsql;

-- Replaces diesel_set_updated_at() because system fields are namespaced.
CREATE OR REPLACE FUNCTION xr_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.xr_updated_at IS NOT DISTINCT FROM OLD.xr_updated_at
    ) THEN
        NEW.xr_updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
