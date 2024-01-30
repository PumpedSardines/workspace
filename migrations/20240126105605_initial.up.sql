-- Add up migration script here

CREATE TABLE workspaces (
  name TEXT NOT NULL PRIMARY KEY,
  path TEXT NOT NULL
);
