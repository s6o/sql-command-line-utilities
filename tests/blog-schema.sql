CREATE TABLE entries (
  id INTEGER PRIMARY KEY AUTOINCREMENT
  , created_at_utc TEXT DEFAULT (strftime('%Y-%m-%d %H:%m:%S.%s', 'now', 'utc'))
  , title TEXT NOT NULL
  , post TEXT
  , word_count INTEGER DEFAULT 0
);

CREATE INDEX entry_timestamp_index ON entries (created_at_utc);

CREATE TABLE tags (
  name TEXT PRIMARY KEY
  , created_at_utc TEXT DEFAULT (strftime('%Y-%m-%d %H:%m:%S.%s', 'now', 'utc'))
);

CREATE INDEX tag_timestamp_index ON tags (created_at_utc);

CREATE TABLE entry_tags (
  id INTEGER REFERENCES entries(id) ON UPDATE CASCADE
  , name TEXT REFERENCES tags(name) ON UPDATE CASCADE
  , PRIMARY KEY (id, name)
);
