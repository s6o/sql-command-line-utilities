# SQLite Command Line Utilities
Command line utilities to make working with SQLite databases more convenient
in file system utilities (e.g. ls) like manner.

  * sqls - for listing schema (tables) and table contents
  * sqle - for inserting and updating data in tables
  * sqld - for deleting data from tables

## Mapping tables and columns to a path

### Root
The root path / will correspond to the sqlite_master table.

### Path

```
{/[<table name>[-<explicit primary keys>][/<filter>][/<columns>]}

<explicit primary keys> = {<column name>[,...]}

<filter> = {<column name>=<value>[<operator>...]}

<operator> = [ & | | ]

<columns> = {<column name>[,...]}
```

## sqls
List schema and content of a SQLite database via a filesystem like path.

Usage:

```
$ sqls [OPTIONS] <db-file> <path>
```

Given the following schema in blog.db:

```
CREATE TABLE entries (
  id PRIMARY KEY
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
```

### Listing schema/tables
Instead of:

```
$ echo "SELECT tbl_name FROM sqlite_master WHERE type = 'table';" | sqlite3 blog.db
```

to get list of table name separated by space:

```
$ sqls blog.db /
entries tags entry_tags
```

or to get a list of table names one per each line:

```
$ sqls -1 blog.db /
entries
tags
entry_tags
```

or to get a listing of the full schema in SQL:

```
$ sqls -s blog.db /
CREATE TABLE entries (
  id PRIMARY KEY
  , created_at_utc TEXT DEFAULT (strftime('%Y-%m-%d %H:%m:%S.%s', 'now', 'utc'))
  , title TEXT NOT NULL
  , post TEXT
  , word_count INTEGER DEFAULT 0
)
CREATE INDEX entry_timestamp_index ON entries (created_at_utc)
CREATE TABLE tags (
  name TEXT PRIMARY KEY
  , created_at_utc TEXT DEFAULT (strftime('%Y-%m-%d %H:%m:%S.%s', 'now', 'utc'))
)
CREATE INDEX tag_timestamp_index ON tags (created_at_utc)
CREATE TABLE entry_tags (
  id INTEGER REFERENCES entries(id) ON UPDATE CASCADE
  , name TEXT REFERENCES tags(name) ON UPDATE CASCADE
  , PRIMARY KEY (id, name)
)
```

or to list a schema of single entity:

```
$sqls -s blog.db /entries
CREATE TABLE entries (
  id PRIMARY KEY
  , created_at_utc TEXT DEFAULT (strftime('%Y-%m-%d %H:%m:%S.%s', 'now', 'utc'))
  , title TEXT NOT NULL
  , post TEXT
  , word_count INTEGER DEFAULT 0
)
```

### Listing content from a table

Instead of:

```
$ echo "SELECT * FROM tags;" | sqlite3 blog.db
```

to get all records with all columns for every record:

```
$ sqls blog.db /tags
algorithms 2016-04-14 13:04:05.1460640725
argriculture 2016-04-14 13:04:05.1460640725
bahamas 2016-04-14 13:04:30.1460640750
crud 2016-04-14 13:04:37.1460640877
command-line 2016-04-14 13:04:37.1460640877
database 2016-04-14 13:04:37.1460640877
sqlite 2016-04-14 13:04:00.1460640900
sqlite3 2016-04-14 13:04:00.1460640900
json 2016-04-14 13:04:26.1460640926
messagepack 2016-04-14 13:04:26.1460640926
vim 2016-04-14 13:04:26.1460640926
```

with table header:

```
$ sqls -h blog.db /tags
name created_at_utc
algorithms 2016-04-14 13:04:05.1460640725
argriculture 2016-04-14 13:04:05.1460640725
bahamas 2016-04-14 13:04:30.1460640750
crud 2016-04-14 13:04:37.1460640877
command-line 2016-04-14 13:04:37.1460640877
database 2016-04-14 13:04:37.1460640877
sqlite 2016-04-14 13:04:00.1460640900
sqlite3 2016-04-14 13:04:00.1460640900
json 2016-04-14 13:04:26.1460640926
messagepack 2016-04-14 13:04:26.1460640926
vim 2016-04-14 13:04:26.1460640926
```

Applying a where clause:

```
$ sqls blog.db /tags/name="sqlite"
sqlite 2016-04-14 13:04:00.1460640900
```
or with custom column separator:

```
$ sqls --colsep=" | " blog.db /tags/name="sqlite"
sqlite | 2016-04-14 13:04:00.1460640900
```

## sqle
Edit content in a SQLite database via a filesystem like path that addresses
a row or a single cell.

Usage:

```
$ sqle <db-file> <path> <new-content>
```

### Inserting data

To insert a new row, explicit primary key and all columns:

```
$ sqle blog.db /entries 1,"2016-04-14 13:04:26.1460640926","Some Thoughts","Lorem ipsum",2
```

To insert a new row, with auto-incremented primary key and select columns:

```
$ sqle blog.db /entries-id/title,post,word_count "Another Day","Same old, same old",4
```

To insert a new row, with expicit primary key and select columns:

```
$ sqle blog.db /entries/id,title,post,word_count 3,"Everything","42",1
```

### Updating records

Update a column value with a primary key in the where clause:

```
$ sqle blog.db /entries/id=1 "Another Great Day"
```

Update several columns where filter matches:

```
$ sqle blog.db /entries/id=1&title="Another Great Day"/title,post "An Actual Day","Stuff that actually happened"
```

## sqld
Remove content from table or database.

Usage:

```
$ sqld <db-file> <path>
```

Instead of:

```
$ echo "DELETE FROM tags WHERE name = 'sqlite';" | sqlite3 blog.db
```

do

```
$ sqld blog.db /tags/name="sqlite"
```
