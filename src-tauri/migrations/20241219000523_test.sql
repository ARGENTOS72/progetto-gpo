-- Add migration script here
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email VARCHAR(255),
    password VARCHAR(255)
);

INSERT INTO users (email, password) VALUES ('email@test', 'password');