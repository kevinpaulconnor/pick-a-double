-- Add migration script here
-- Create Picks Table
CREATE TABLE picks (
  id uuid NOT NULL,
  PRIMARY KEY (id),
  user_id uuid NOT NULL,
  game_id uuid NOT NULL,
  player_id uuid NOT NULL,
  created_at timestamptz NOT NULl
);