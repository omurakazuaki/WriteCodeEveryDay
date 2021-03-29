-- Your SQL goes here
CREATE TABLE users (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL
);

ALTER TABLE posts
ADD user_id INTEGER references users(id);
