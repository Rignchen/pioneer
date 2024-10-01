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

