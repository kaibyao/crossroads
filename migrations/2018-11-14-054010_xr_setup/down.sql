DROP EXTENSION pgcrypto;

DROP SEQUENCE IF EXISTS xr_next_id_seq;
DROP FUNCTION IF EXISTS xr_next_id();

DROP FUNCTION IF EXISTS xr_setup_table(_tbl regclass);
