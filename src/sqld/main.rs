extern crate clap;
use clap::{Arg, App};

//extern crate rusqlite;
//use rusqlite::Connection;

extern crate sqlcmdlutils;
use sqlcmdlutils::dbpath::{DbPath, db_path_help, parse_db_path};


fn main() {
    let arg_matches =
        App::new("sqld - delete SQLite database contents")
            .version(env!("CARGO_PKG_VERSION"))
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

//    let db_file = arg_matches.value_of("db_file").unwrap();
//    let conn = Connection::open(db_file).unwrap();

    match parse_db_path(db_path) {
        DbPath::Error(msg) => println!("{}", msg),
        DbPath::Root => println!("To be implemented for root"),
        DbPath::Table(table) => println!("To be implemented for table {}", table),
        DbPath::TableFilter {table, filter} => println!("To be implemented for {} and {}", table, filter),
        DbPath::TableSelect {table, select} => println!("To be implemented for {} and {}", table, select),
        DbPath::TableFilterSelect {table, filter, select} => println!("To be implemented for {}, {} and {}", table, filter, select),
    }
}