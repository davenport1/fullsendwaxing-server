CREATE TABLE IF NOT EXISTS users (
  id          SERIAL PRIMARY KEY,
  email       VARCHAR(64) NOT NULL UNIQUE,
  first_name  VARCHAR(64) NOT NULL,
  last_name   VARCHAR(64) NOT NULL,
  password    VARCHAR(64) NOT NULL,
  deleted_at  TIMESTAMPTZ DEFAULT NULL,
  role        INTEGER NOT NULL,
  token       TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS appointments (
  id          SERIAL PRIMARY KEY,
  user_id     INTEGER NOT NULL,
  date        TIMESTAMPTZ NOT NULL,
  category    INTEGER,
  notes       TEXT,
  CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS blogs (
  id            SERIAL PRIMARY KEY,
  title         VARCHAR(255) NOT NULL,
  post_date  TIMESTAMPTZ DEFAULT NULL,
  body   TEXT DEFAULT NULL,
  deleted_at    TIMESTAMPTZ DEFAULT NULL,
  user_id       INTEGER DEFAULT NULL,
  CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT INTO users (email, first_name, last_name, password, role)
    VALUES ('admin@admin.com', 'matt', 'davenport', 'password', 1);

INSERT INTO users (email, first_name, last_name, password, role)
    VALUES ('user@user.com', 'regular', 'user', 'password', 2);

INSERT INTO users (email, first_name, last_name, password, role)
    VALUES ('user2@user.com', 'regular2', 'user2', 'password', 2);

INSERT INTO blogs (title, post_date, user_id) VALUES (
  'my first post',
  NOW(),
  (select id from users where email = 'admin@admin.com')
);

INSERT INTO appointments (user_id, date, category, notes) 
VALUES (
    1, 
    TIMESTAMP WITH TIME ZONE '2024-01-29 12:34:56.789123+00:00', 
    1, 
    'Full board tune'
);