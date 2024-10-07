pub enum DatabaseConnection {
    Postgres(postgres::Client),
    Sqlite(sqlite::Connection),
    Mysql(mysql::Pool),
}

