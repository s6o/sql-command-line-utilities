extern crate clap;
use clap::{Arg, App};

extern crate rusqlite;
use rusqlite::Connection;

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
            let mut stmt = conn.prepare(query).unwrap();
            let mut rows = stmt.query(&[]).unwrap();

            while let Some(result_row) = rows.next() {
                match result_row {
                    Ok(row) => {
                        let item: String = row.get::<i32, String>(0);
                        println!("{}", item);
                    },
                    Err(msg) => println!("{}", msg)
                }
            }
        }

        DbPath::Table(table) => {
            match arg_matches.occurrences_of("schema") {
                1 => {
                    let query = "SELECT sql FROM sqlite_master WHERE tbl_name = $1 AND length(sql) > 0";
                    let mut stmt = conn.prepare(query).unwrap();
                    let mut rows = stmt.query(&[&table]).unwrap();

                    while let Some(result_row) = rows.next() {
                        match result_row {
                            Ok(row) => {
                                let item: String = row.get::<i32, String>(0);
                                println!("{}", item);
                            },
                            Err(msg) => println!("{}", msg)
                        }
                    }
                }
                _ => {
                    let checked_table =
                        table.replace("'", "''").replace("\"", "\\\"").replace(";", "");
                    let query = format!("SELECT * FROM {}", checked_table);
                    let mut stmt = conn.prepare(query.as_str()).unwrap();
                    let mut rows = stmt.query(&[]).unwrap();

                    while let Some(result_row) = rows.next() {
                        match result_row {
                            Ok(row) => {
                                for col_idx in 0..row.column_count() {
                                    print!("{} ", row.get::<i32, String>(col_idx));
                                }
                                println!("");
                            },
                            Err(msg) => println!("{}", msg)
                        }
                    }
                }
            };
        }

        DbPath::TableFilter {table, filter} => println!("To implemented for {} and {}", table, filter),
        DbPath::TableSelect {table, select} => println!("To implemented for {} and {}", table, select),
        DbPath::TableFilterSelect {table, filter, select} => println!("To implemented for {}, {} and {}", table, filter, select),
    }
}