# SQLite Command Line Utilities
Command line utilities to make working with SQLite databases more convenient
in file system utilities (e.g. ls) like manner.

  * sqls - for listing schema (tables) and table contents
  * sqle - for inserting and updating data in tables
  * sqld - for deleting data from tables

Currently a small project to learn Rust. Actual practical value yet to be determined ...

## Mapping tables and columns to a path

### Root
The root path / will correspond to the sqlite_master table.

### Path

```
path := ['"']'/'[table['/'columns]['/'filter]]['"']

table := <text>

columns := [column][['=']value][','columns]
column := <text>
value := ['\'']<text>['\'']

filter := column comparator value [operator filter]
comparator := '==' | '!=' | '<=' | '>=' | '<' | '>'
operator := '&' | '|'
```

## sqls
List schema and content of a SQLite database via a filesystem like path.

Usage:

```
$ sqls [OPTIONS] <path> <db-file>
```

Given the following schema in blog.db:

```
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
```

### Listing schema/tables
Instead of:

```
$ echo "SELECT tbl_name FROM sqlite_master WHERE type = 'table';" | sqlite3 blog.db
```

to get list of table names:

```
$ sqls / blog.db
entries
tags
entry_tags
```

or to get a listing of the full schema in SQL:

```
$ sqls -s / blog.db
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
$sqls -s /entries blog.db
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
$ sqls /tags blog.db
algorithms|2016-04-14 13:04:05.1460640725
argriculture|2016-04-14 13:04:05.1460640725
bahamas|2016-04-14 13:04:30.1460640750
crud|2016-04-14 13:04:37.1460640877
command-line|2016-04-14 13:04:37.1460640877
database|2016-04-14 13:04:37.1460640877
sqlite|2016-04-14 13:04:00.1460640900
sqlite3|2016-04-14 13:04:00.1460640900
json|2016-04-14 13:04:26.1460640926
messagepack|2016-04-14 13:04:26.1460640926
vim|2016-04-14 13:04:26.1460640926
```

with table header:

```
$ sqls -h /tags blog.db
name|created_at_utc
algorithms|2016-04-14 13:04:05.1460640725
argriculture|2016-04-14 13:04:05.1460640725
bahamas|2016-04-14 13:04:30.1460640750
crud|2016-04-14 13:04:37.1460640877
command-line|2016-04-14 13:04:37.1460640877
database|2016-04-14 13:04:37.1460640877
sqlite|2016-04-14 13:04:00.1460640900
sqlite3|2016-04-14 13:04:00.1460640900
json|2016-04-14 13:04:26.1460640926
messagepack|2016-04-14 13:04:26.1460640926
vim|2016-04-14 13:04:26.1460640926
```

Applying a where clause:

```
$ sqls "/tags/name=='sqlite'" blog.db
sqlite|2016-04-14 13:04:00.1460640900
```

or

```
$ sqls -d " | " "/tags/name=='sqlite'" blog.db
sqlite | 2016-04-14 13:04:00.1460640900
```

$

## sqle
Edit content in a SQLite database via a filesystem like path that addresses
a row or a single cell.

Usage:

```
$ sqle <path> <db-file>
```

### Inserting data

To insert a new row, explicit primary key and all columns:

```
$ sqle "/entries/1,'2016-04-14 13:04:26.1460640926','Some Thoughts','Lorem ipsum',2" blog.db
```

To insert a new row, with auto-incremented primary key and select columns (the
auto-incremented primary keys are just omitted):

```
$ sqle "/entries/title='Another Day',post='Same old, same old',word_count=3" blog.db
```

To insert a new row, with expicit primary key and select columns:

```
$ sqle "/entries/id=3,title='Everything',post='42',word_count=1" blog.db
```

### Updating records

Update a column value with a primary key in the where clause:

```
$ sqle "/entries/title='Another Great Day'/id==1" blog.db
```

Update several columns where filter matches:

```
$ sqle "/entries/title='An Actual Day',post='Stuff that actually happened'/id==1&title=='Another Great Day'" blog.db
```

## sqld
Remove content from table or database.

Usage:

```
$ sqld <path> <db-file>
```

Instead of:

```
$ echo "DELETE FROM tags WHERE name = 'sqlite';" | sqlite3 blog.db
```

do

```
$ sqld "/tags/name=='sqlite'" blog.db
```
