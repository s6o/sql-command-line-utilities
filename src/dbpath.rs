pub enum DbPath {
    Root,
    Table(String),
    TableColumns {table: String, columns: String},
    TableFilter {table: String, filter: String},
    TableColumnsFilter {table: String, columns: String, filter: String},
    Error(String)
}

pub fn db_path_help<'a>() -> &'a str {
    "path := ['\"']'/'[table['/'columns]['/'filter]]['\"']\n\n\
    table := <text>\n\n\
    columns := [column][['=']value][','columns]\n\
    column := <text>\n\
    value := ['\'']<text>['\'']\n\n\
    filter := column comparator value [operator filter]\n\
    comparator := '==' | '!=' | '<=' | '>=' | '<' | '>'\n\
    operator := '&' | '|'"
}

pub fn parse_db_path(path: &str) -> DbPath {
    if path.starts_with("/") {
        let v: Vec<&str> = path.split("/").collect();
        let parts: Vec<&str> = v.into_iter().filter(|s| s.len() > 0).collect();
        if parts.is_empty() {
            DbPath::Root
        } else if parts.len() == 1 {
            DbPath::Table(escape(parts[0]))
        } else if parts.len() == 2 {
            if is_select(parts[1]) {
                DbPath::TableColumns {
                    table: escape(parts[0]),
                    columns: escape(parts[1])
                }
            } else if is_filter(parts[1]) {
                DbPath::TableFilter {
                    table: escape(parts[0]),
                    filter: parse_filter(parts[1])
                }
            } else {
                DbPath::Error("Incorrect /<filter> or /<columns> specification.".to_string())
            }
        } else if parts.len() == 3 {
            if is_select(parts[1]) && is_filter(parts[2]) {
                DbPath::TableColumnsFilter {
                    table: escape(parts[0]),
                    columns: escape(parts[1]),
                    filter: parse_filter(parts[2]),
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
    s.contains("==")
}

fn is_select(s: &str) -> bool {
    s.contains(",") && (! s.contains("=="))
}

fn escape(s: &str) -> String {
    s.replace("'", "''").replace("\"", "\\\"").replace(";", "")
}

fn parse_filter(filter: &str) -> String {
    filter.replace(";", "")
      .replace("==", "=")
      .replace("!=", "<>")
      .replace("&", " AND ")
      .replace("|", " OR ")
}
