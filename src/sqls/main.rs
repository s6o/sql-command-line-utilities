extern crate clap;
use clap::{Arg, App};

extern crate rusqlite;
use rusqlite::{Connection, Rows, Row};
use rusqlite::types::{ToSql, Value};

extern crate sqlcmdlutils;
use sqlcmdlutils::dbpath::{DbPath, db_path_help, parse_db_path};


fn main() {
    let arg_matches =
        App::new("sqls - list SQLite database contents")
            .version(env!("CARGO_PKG_VERSION"))
            .arg(Arg::with_name("schema")
                .short("s")
                .long("schema")
                .help("List schema. For use with a root or table path."))
            .arg(Arg::with_name("path")
                .help(db_path_help())
                .required(true)
                .index(1))
            .arg(Arg::with_name("db_file")
                .help("Absolute or relative path to an SQLite database file.")
                .value_name("database")
                .required(true)
                .index(2))
            .get_matches();

    let db_path = arg_matches.value_of("path").unwrap();

    let db_file = arg_matches.value_of("db_file").unwrap();
    let conn = Connection::open(db_file).unwrap();

    match parse_db_path(db_path) {
        DbPath::Error(msg) => println!("{}", msg),

        DbPath::Root => {
            let query: &str = match arg_matches.occurrences_of("schema") {
                0 => "SELECT tbl_name FROM sqlite_master WHERE type = 'table' AND length(sql) > 0",
                _ => "SELECT sql FROM sqlite_master WHERE length(sql) > 0",
            };
            compose_results(conn, query, &[], &output_column);
        }

        DbPath::Table(table) => {
            match arg_matches.occurrences_of("schema") {
                1 => {
                    let query = "SELECT sql FROM sqlite_master WHERE tbl_name = $1 AND length(sql) > 0";
                    compose_results(conn, query, &[&table], &output_column);
                }
                _ => {
                    let query = format!("SELECT * FROM {}", table);
                    compose_results(conn, query.as_str(), &[], &output_rows);
                }
            };
        }

        DbPath::TableColumns {table, columns} => {
            let query = format!("SELECT {} FROM {}", columns, table);
            compose_results(conn, query.as_str(), &[], &output_rows);
        }

        DbPath::TableFilter {table, filter} => {
            let query = format!("SELECT * FROM {} WHERE {}", table, filter);
            compose_results(conn, query.as_str(), &[], &output_rows);
        }

        DbPath::TableColumnsFilter {table, columns, filter} => {
            let query = format!("SELECT {} FROM {} WHERE {}", columns, table, filter);
            compose_results(conn, query.as_str(), &[], &output_rows);
        }
    }
}

fn compose_results(conn: Connection, query: &str, params: &[&ToSql], output: &Fn(Rows) -> ()) -> () {
    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt.query(params).unwrap();
    output(rows);
}

fn output_column(mut rows: Rows) -> () {
    while let Some(result_row) = rows.next() {
        match result_row {
            Ok(row) => println!("{}", convert_value(&row, 0)),
            Err(msg) => println!("{}", msg)
        }
    }
}

fn output_rows(mut rows: Rows) -> () {
    while let Some(result_row) = rows.next() {
        match result_row {
            Ok(row) => {
                let cc = row.column_count();
                for col_idx in 0..cc {
                    print!("{}{}"
                        , convert_value(&row, col_idx)
                        , if col_idx < cc - 1 { "|" } else { "" }
                    );
                }
                println!("");
            },
            Err(msg) => println!("{}", msg)
        }
    }
}

fn convert_value(row: &Row, col_idx: i32) -> String {
    match row.get_checked::<i32, Value>(col_idx) {
        Ok(v) => {
            match v {
                Value::Null => "<null>".to_string(),
                Value::Integer(i) => i.to_string(),
                Value::Real(f) => f.to_string(),
                Value::Text(t) => t.to_string(),
                Value::Blob(_) => "<blob>".to_string()
            }
        }
        Err(_) => "<type_fail>".to_string()
    }
}