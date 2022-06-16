CREATE TABLE auth.user (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR NOT NULL,
    password TEXT NOT NULL,
    UNIQUE(username)
);
