-- Create users Table
CREATE TABLE users (
  id uuid NOT NULL,
  PRIMARY KEY (id),
  email TEXT NOT NULL UNIQUE,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  created_at timestamptz NOT NULl
);

-- not ready to support this constraint yet
-- ALTER TABLE picks ADD CONSTRAINT picksfk FOREIGN KEY (user_id) REFERENCES users (id) MATCH FULL;