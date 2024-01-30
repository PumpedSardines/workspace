-- Add up migration script here

CREATE TABLE legacy_move_data (
  id INTEGER PRIMARY KEY,
  has_moved INTEGER NOT NULL
);
