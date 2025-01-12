CREATE TABLE teams (
  id uuid NOT NULL,
  PRIMARY KEY (id),
  alias TEXT NOT NULL,
  market TEXT NOT NULL,
  name TEXT NOT NULL
);