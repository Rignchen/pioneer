use clap::Parser;
use crate::{
    error::{
        PioneerResult,
        PioneerError,
        PioneerErrorType,
    },
    database::connection::DatabaseConnection,
};

structstruck::strike! {
    #[strikethrough[derive(Parser, Debug, Clone)]]
    pub enum DatabaseType {
        Postgres(struct Postgres {
            pub host: String,
            pub port: u16,
            user: String,
            password: String,
        }),
        Sqlite(struct Sqlite {
            pub file: String,
        }),
        Mysql(struct Mysql {
            pub host: String,
            pub port: u16,
            user: String,
            password: String,
        }),
    }
}

impl DatabaseType {
    fn get_url(&self) -> String {
        match self {
            DatabaseType::Postgres(Postgres {
                host,
                port,
                user,
                password,
            }) => {
                format!("postgres://{}:{}@{}:{}/", user, password, host, port)
            }
            DatabaseType::Sqlite(Sqlite { file }) => {
                format!("{}", file)
            }
            DatabaseType::Mysql(Mysql {
                host,
                port,
                user,
                password,
            }) => {
                format!("mysql://{}:{}@{}:{}/", user, password, host, port)
            }
        }
    }
    pub fn connect(&self) -> PioneerResult<DatabaseConnection> {
        let url = self.get_url();
        match match self {
            DatabaseType::Postgres(_) =>
                match postgres::Client::connect(&url, postgres::NoTls) {
                    Ok(client) => Ok(DatabaseConnection::Postgres(client)),
                    Err(e) => Err(e.to_string()),
                }
            DatabaseType::Sqlite(_) =>
                match sqlite::Connection::open(&url) {
                    Ok(client) => Ok(DatabaseConnection::Sqlite(client)),
                    Err(e) => Err(e.to_string()),
                }
            DatabaseType::Mysql(_) =>
                match mysql::Pool::new(url.as_str()) {
                    Ok(client) => Ok(DatabaseConnection::Mysql(client)),
                    Err(e) => Err(e.to_string()),
                }
        } {
            Ok(connection) => Ok(connection),
            Err(e) => Err(PioneerError {
                _pioneer_error: PioneerErrorType::ConnectionError(self.clone()),
                message: e,
            }),
        }
    }
}
