use chrono::Utc;
use gw2_api_models::models::matchup_overview::MatchupOverview;

#[derive(Debug, Clone)]
pub struct OracleAdapterConfig {
    username: String,
    password: String,
    connect_string: String,
}

#[derive(Debug, Clone)]
pub struct OracleAdapter {
    config: OracleAdapterConfig,
}

impl OracleAdapter {
    pub fn new(host: &str, user: &str, password: &str) -> Self {
        let config = OracleAdapterConfig {
            username: user.to_owned(),
            password: password.to_owned(),
            connect_string: host.to_owned(),
        };
        Self { config }
    }

    pub async fn get_connection(&self) -> Result<OracleClientAdapter, oracle::Error> {
        let conn = oracle::Connection::connect(
            &self.config.username,
            &self.config.password,
            &self.config.connect_string,
        )?;
        Ok(OracleClientAdapter { conn })
    }
}

pub struct OracleClientAdapter {
    conn: oracle::Connection,
}

impl OracleClientAdapter {
    pub fn new(conn: oracle::Connection) -> Self {
        Self { conn }
    }

    async fn match_exists_statement(&self) -> Result<oracle::Statement, oracle::Error> {
        self.conn
            .statement(
                "
                SELECT
                    CAST(COUNT(1) AS NUMBER(1))
                FROM
                    DUAL
                WHERE
                    EXISTS (
                        SELECT
                            *
                        FROM
                            MATCHUP_INFOS
                        WHERE
                                MATCHUP_INFOS.MATCHUP_ID = :1
                            AND MATCHUP_INFOS.INITIAL_DATE_MATCHUP = :2
                    )",
            )
            .build()
    }

    async fn match_exists(&self, data: &MatchupOverview) -> Result<bool, oracle::Error> {
        let mut prepared = self.match_exists_statement().await?;
        let id: &str = data.id();
        let initial_date_matchup: &chrono::DateTime<Utc> = data.start_time();
        let result = prepared.query_row(&[&id, initial_date_matchup])?;
        let exists: u64 = result.get(0)?;
        Ok(exists > 0)
    }

    async fn insert_prepared_statement(&self) -> Result<oracle::Statement, oracle::Error> {
        self.conn
            .statement(
                "
                    INSERT INTO 
                        MATCHUP_INFOS 
                        (MATCHUP_ID, INITIAL_DATE_MATCHUP, END_DATE_MATCHUP, INFO) 
                    VALUES (:1, :2, :3, :4)",
            )
            .build()
    }

    pub async fn insert(&self, data: &MatchupOverview) -> Result<(), oracle::Error> {
        if self.match_exists(data).await? {
            return self.update(data).await;
        }
        let mut statement = self.insert_prepared_statement().await?;
        let id: &str = data.id();
        let initial_date_matchup: &chrono::DateTime<Utc> = data.start_time();
        let end_date_matchup: &chrono::DateTime<Utc> = data.end_time();
        let info: String = serde_json::to_string(data.skirmishes()).unwrap();
        let info_str: &str = &(info);
        statement.execute(&[&id, initial_date_matchup, end_date_matchup, &info_str])
    }

    async fn update_statement(&self) -> Result<oracle::Statement, oracle::Error> {
        self.conn
            .statement(
                "
                UPDATE 
                    MATCHUP_INFOS 
                SET 
                    INFO = :1 
                WHERE 
                    MATCHUP_ID = :2 
                AND INITIAL_DATE_MATCHUP = :3",
            )
            .build()
    }

    async fn update(&self, data: &MatchupOverview) -> Result<(), oracle::Error> {
        let mut prepared = self.update_statement().await?;
        let id: &str = data.id();
        let initial_date_matchup: &chrono::DateTime<Utc> = data.start_time();
        let info: String = serde_json::to_string(data.skirmishes()).unwrap();
        let info_str: &str = &info;
        prepared.execute(&[&info_str, &id, initial_date_matchup])
    }

    #[allow(dead_code)]
    async fn get_table_names(&self) -> Result<Vec<(String, String)>, oracle::Error> {
        let mut statment = self
            .conn
            .statement(
                "
                    SELECT
                        TABLE_NAME,
                        OWNER
                    FROM
                        ALL_TABLES
                    ORDER BY
                        OWNER,
                        TABLE_NAME",
            )
            .build()?;
        let result = statment.query_as::<(String, String)>(&[])?;
        let mut names: Vec<(String, String)> = vec![];
        for row in result {
            let tuple = row?;
            names.push(tuple);
        }
        Ok(names)
    }

    #[allow(dead_code)]
    async fn select(&self) -> Result<(), oracle::Error> {
        let mut statment = self
            .conn
            .statement(
                "
                    SELECT
                        *
                    FROM
                        MATCHUP_INFOS",
            )
            .build()?;
        let result = statment.query(&[])?;
        dbg!(&result);
        for row in result {
            let tuple = row?;
            dbg!(&tuple);
            let id: String = tuple.get(0)?;
            let start: String = tuple.get(1)?;
            let end: String = tuple.get(2)?;
            println!("{:?} {:?} {:?}", id, start, end)
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use gw2_api_models::models::matchup_overview::mock;

    use crate::OracleAdapter;

    #[tokio::test]
    async fn can_connect() -> Result<(), oracle::Error> {
        dotenv::dotenv().ok();
        let host: &str = &env::var("ORACLE_CONN")
            .expect("CONN must be set.")
            .to_owned();
        let user: &str = &env::var("ORACLE_USER")
            .expect("USER must be set.")
            .to_owned();
        let password: &str = &env::var("ORACLE_PASSWORD")
            .expect("PASSWORD must be set.")
            .to_owned();

        let adapter = OracleAdapter::new(host, user, password);
        let client = adapter.get_connection().await?;

        let names = client.get_table_names().await?;
        let filtered_names: Vec<&(String, String)> = names
            .iter()
            .filter(|(_table_name, owner)| owner.eq("ADMIN"))
            .collect();
        dbg!(filtered_names);
        // for o in obj {
        //     client.insert(o).await?;
        // }

        Ok(())
    }

    #[tokio::test]
    async fn can_check_if_data_exists() -> Result<(), oracle::Error> {
        dotenv::dotenv().ok();
        let host: &str = &env::var("ORACLE_CONN")
            .expect("CONN must be set.")
            .to_owned();
        let user: &str = &env::var("ORACLE_USER")
            .expect("USER must be set.")
            .to_owned();
        let password: &str = &env::var("ORACLE_PASSWORD")
            .expect("PASSWORD must be set.")
            .to_owned();

        let adapter = OracleAdapter::new(host, user, password);
        let client = adapter.get_connection().await?;

        let exists = client.match_exists(&mock::get_naive_mock()).await?;
        dbg!(exists);
        // for o in obj {
        //     client.insert(o).await?;
        // }

        Ok(())
    }
    #[tokio::test]
    async fn can_select_all() -> Result<(), oracle::Error> {
        dotenv::dotenv().ok();
        let host: &str = &env::var("ORACLE_CONN")
            .expect("CONN must be set.")
            .to_owned();
        let user: &str = &env::var("ORACLE_USER")
            .expect("USER must be set.")
            .to_owned();
        let password: &str = &env::var("ORACLE_PASSWORD")
            .expect("PASSWORD must be set.")
            .to_owned();

        let adapter = OracleAdapter::new(host, user, password);
        let client = adapter.get_connection().await?;

        client.select().await?;

        Ok(())
    }
}
