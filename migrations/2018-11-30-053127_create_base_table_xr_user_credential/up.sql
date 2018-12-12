-- This file builds the table to store xr_user credentials.
-- passwords are not saved in plain-text, rather they are encrypted
-- with pgcrypto. For details, see
-- https://www.meetspaceapp.com/2016/04/12/passwords-postgresql-pgcrypto.html

-- If we were to ever do any kind of SSO integrations (google, facebook, openID, etc), those user login tokens would probably be stored here.

SELECT xr_setup_table('xr_user_credential');

ALTER TABLE xr_user_credential
  ADD COLUMN xr_user_id BIGINT NOT NULL REFERENCES xr_user,
  ADD COLUMN xr_password text NOT NULL,
  ADD CONSTRAINT unique_user_id UNIQUE (xr_user_id)
;
