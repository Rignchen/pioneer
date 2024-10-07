use clap::Parser;
use crate::error::{
    PioneerResult,
    PioneerError,
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

pub enum DatabaseConnection {
    Postgres(postgres::Client),
    Sqlite(sqlite::Connection),
    Mysql(mysql::Pool),
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
                format!("sqlite://{}", file)
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
        match self {
            DatabaseType::Postgres(_) =>
                match postgres::Client::connect(&url, postgres::NoTls) {
                    Ok(client) => Ok(DatabaseConnection::Postgres(client)),
                    Err(_) => Err(PioneerError::ConnectionError(self.clone())),
                }
            DatabaseType::Sqlite(_) =>
                match sqlite::Connection::open(&url) {
                    Ok(client) => Ok(DatabaseConnection::Sqlite(client)),
                    Err(_) => Err(PioneerError::ConnectionError(self.clone())),
                }
            DatabaseType::Mysql(_) =>
                match mysql::Pool::new(url.as_str()) {
                    Ok(client) => Ok(DatabaseConnection::Mysql(client)),
                    Err(_) => Err(PioneerError::ConnectionError(self.clone())),
                }
        }
    }
}
