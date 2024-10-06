use clap::Parser;

structstruck::strike! {
    #[strikethrough[derive(Parser, Debug)]]
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
}
