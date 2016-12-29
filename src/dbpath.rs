pub enum DbPath {
    Root,
    Table(String),
    TableFilter {table: String, filter: String},
    TableSelect {table: String, select: String},
    TableFilterSelect {table: String, filter: String, select: String},
    Error(String)
}

pub fn db_path_help<'a>() -> &'a str {
    "/[<table name>[/<filter>][/<columns>]\n\n\
    <filter> = {<column name>=<value>[<operator>...]}\n\
    <operator> = [ & | ]\n\
    <columns> = {<column name>[,...]}\n\n"
}

pub fn parse_db_path(path: &str) -> DbPath {
    if path.starts_with("/") {
        let v: Vec<&str> = path.split("/").collect();
        let parts: Vec<&str> = v.into_iter().filter(|s| s.len() > 0).collect();
        if parts.is_empty() {
            DbPath::Root
        } else if parts.len() == 1 {
            DbPath::Table(parts[0].to_string())
        } else if parts.len() == 2 {
            if is_filter(parts[1]) {
                DbPath::TableFilter {
                    table: parts[0].to_string(),
                    filter: parts[1].to_string()
                }
            } else if is_select(parts[1]) {
                DbPath::TableSelect {
                    table: parts[0].to_string(),
                    select: parts[1].to_string()
                }
            } else {
                DbPath::Error("Incorrect /<filter> or /<columns> specification.".to_string())
            }
        } else if parts.len() == 3 {
            if is_filter(parts[1]) && is_select(parts[2]) {
                DbPath::TableFilterSelect {
                    table: parts[0].to_string(),
                    filter: parts[1].to_string(),
                    select: parts[2].to_string()
                }
            } else {
                DbPath::Error("Incorrect /<filter> or /<columns> specification.".to_string())
            }
        } else {
            DbPath::Error("Incorrect <path>, at most 3 parts, separated by / (slash) are allowed.".to_string())
        }
    } else {
        DbPath::Error("The <path> must start with a / (slash)".to_string())
    }
}

fn is_filter(s: &str) -> bool {
    s.contains("=")
}

fn is_select(s: &str) -> bool {
    s.contains(",")
}
