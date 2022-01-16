-- Your SQL goes here
CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  username VARCHAR(255) NOT NULL UNIQUE,
  encrypted_password VARCHAR(1000) NOT NULL,
  follower_count INT NOT NULL,
  following_count INT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


SELECT diesel_manage_updated_at('users');

CREATE TABLE tweets (
  id BIGSERIAL PRIMARY KEY,
  tweet TEXT NOT NULL,
  user_id BIGINT NOT NULL REFERENCES users,
  like_count INT NOT NULL,
  retweet_count INT NOT NULL,
  comments_count INT NOT NULL,
  retweet_id BIGINT REFERENCES tweets,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


SELECT diesel_manage_updated_at('tweets');


CREATE TABLE follows (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL REFERENCES users,
  follower_id BIGINT NOT NULL REFERENCES users,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


SELECT diesel_manage_updated_at('follows');


CREATE TABLE logins (
  id BIGSERIAL PRIMARY KEY,
  token VARCHAR(100) NOT NULL UNIQUE,
  user_id BIGINT NOT NULL REFERENCES users,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


SELECT diesel_manage_updated_at('logins');
