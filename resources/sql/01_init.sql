CREATE DATABASE rest_api_example;
\c rest_api_example

DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
  id              SERIAL PRIMARY KEY,
  first_name      VARCHAR(100) NOT NULL,
  last_name       VARCHAR(100) NOT NULL,
  email           VARCHAR(255) NOT NULL,
  date_of_birth   DATE NULL,
  created_at      TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at      TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX users_name ON users (first_name, last_name);
CREATE UNIQUE INDEX users_email ON users (email);
CREATE INDEX users_created_at ON users (created_at);
CREATE INDEX users_updated_at ON users (updated_at);

INSERT INTO users (first_name, last_name, email) VALUES('Foo', 'Bar', 'foo.bar@test.com');