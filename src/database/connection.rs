pub enum DatabaseConnection {
    Postgres(postgres::Client),
    Sqlite(sqlite::Connection),
    Mysql(mysql::Pool),
}

impl DatabaseConnection {
    pub fn get_all_tables(&mut self, schema: Option<String>) -> Vec<String> {
        match self {
            DatabaseConnection::Postgres(ref mut client) => {
                // SELECT table_name FROM information_schema.tables WHERE table_schema = $1
                client
                    .query(
                        "SELECT table_name FROM information_schema.tables WHERE table_schema = $1",
                        &[&schema.unwrap_or("public".to_string())],
                    )
                    .unwrap().iter()
                    .map(|row| row.get(0)).collect()
            }
            _ => todo!()
        }
    }
}
