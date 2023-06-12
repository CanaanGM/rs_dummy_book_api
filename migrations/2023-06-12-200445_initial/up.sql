
CREATE TABLE books (
    id Integer primary key autoincrement,
    name varchar not null,
    category varchar not null,
    created_at Timestamp Not null default current_timestamp
)