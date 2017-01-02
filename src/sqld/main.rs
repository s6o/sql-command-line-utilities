extern crate clap;
use clap::{Arg, App};

extern crate rusqlite;
use rusqlite::{Connection, Transaction, TransactionBehavior};
use rusqlite::types::{ToSql};

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

    let db_file = arg_matches.value_of("db_file").unwrap();
    let mut conn = Connection::open(db_file).unwrap();

    match parse_db_path(db_path) {
        DbPath::Error(msg) => println!("{}", msg),

        DbPath::Root => {
            let  tx = Transaction::new(&mut conn, TransactionBehavior::Deferred).unwrap();
            let mut tables: Vec<String> = Vec::new();

            { // prepare available tables
                let query = "SELECT tbl_name FROM sqlite_master WHERE type = 'table' AND length(sql) > 0";
                let mut stmt = tx.prepare(query).unwrap();
                let mut rows = stmt.query(&[]).unwrap();
                while let Some(result_row) = rows.next() {
                    match result_row {
                        Ok(row) => tables.push(row.get::<i32, String>(0)),
                        Err(_) => ()
                    }
                }
            }

            if tables.len() > 0 {
                println!("Are you sure you'd like to delete data from all tables? S(ure)/H(ell no)");
                let mut user_input = String::new();
                match std::io::stdin() .read_line(&mut user_input) {
                    Ok(_) => {
                        let response = user_input.trim().to_lowercase();
                        if response != "s" && response != "sure" {
                            return ()
                        }
                    }
                    Err(msg) => {
                        println!("Error: {}", msg);
                        return ()
                    }
                }

                let mut tx_commit = true;
                for t in tables {
                    let delete_query = format!("DELETE FROM {}", t);
                    match tx.execute(delete_query.as_str(), &[]) {
                        Ok(deleted) => println!("Deleted {} row(s) from {}", deleted, t),
                        Err(msg) => {
                            println!("Delete failed at table: {} - doing rollback ...", t);
                            println!("Delete failure message: {}", msg);
                            tx_commit = false;
                            break;
                        }
                    }
                }
                if tx_commit {
                    match tx.commit() {
                        Ok(_) => (),
                        Err(_) => println!("Failed to commit deletes."),
                    };
                } else {
                    match tx.rollback() {
                        Ok(_) => (),
                        Err(_) => println!("Failed to do rollback for deletes."),
                    }
                }
            } else {
                println!("No tables found in the database.");
            }
        }

        DbPath::Table(table) => {
            let query = format!("DELETE FROM {}", table);
            execute_query(&conn, query.as_str(), table.as_str(), &[]);
        }

        DbPath::TableFilter {table, filter} => {
            let query = format!("DELETE FROM {} WHERE {}", table, filter);
            execute_query(&conn, query.as_str(), table.as_str(), &[]);
        }

        _ => println!("Column selection is not supported for DELETE."),
    }
}

fn execute_query(conn: &Connection, query: &str, table: &str, params: &[&ToSql]) -> () {
    match conn.execute(query, params) {
        Ok(deleted) => println!("Deleted {} row(s) from {}", deleted, table),
        Err(msg) => println!("Delete failed: {}", msg),
    }
}
