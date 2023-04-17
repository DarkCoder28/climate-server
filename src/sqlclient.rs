use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, MySql};
use chrono::{prelude::*};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Entry {
    pub source: String,
    pub time: chrono::DateTime<Utc>,
    #[serde(with = "rust_decimal::serde::float")]
    pub temp_c: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub temp_f: Decimal,
    pub humidity: Option<f32>,
}

#[derive(sqlx::FromRow)]
struct Source {
    source: String,
}

pub async fn get_latest(pool: &Pool<MySql>) -> Result<HashMap<String, Entry>, sqlx::Error> {
    let sources = sqlx::query_as::<_, Source>("SELECT DISTINCT `source` FROM `data` ORDER BY `source`")
        .fetch_all(pool).await?;

    let mut climate_data:HashMap<String, Entry> = HashMap::new();
    for source in sources {
        let source_data = sqlx::query_as::<_, Entry>("SELECT `source`, `time`, `temp_c`, `temp_f`, `humidity` FROM `data` WHERE `source` = ? ORDER BY `time` DESC LIMIT 1")
            .bind(source.source)
            .fetch_one(pool).await?;
        climate_data.insert(String::from(&source_data.source), source_data);
    }

    Ok(climate_data)
}

pub async fn push_new(pool: &Pool<MySql>, data: Vec<Entry>) -> Result<(), sqlx::Error> {
    for entry in data {
        sqlx::query("INSERT INTO `data` (`id`, `source`, `temp_c`, `temp_f`, `humidity`) VALUES (?, ?, ?, ?, ?)")
        .bind(Uuid::new_v4().as_hyphenated())
        .bind(entry.source)
        .bind(entry.temp_c)
        .bind(entry.temp_f)
        .bind(entry.humidity)
        .execute(pool).await?;
    }
    Ok(())
}

pub async fn get_history(pool: &Pool<MySql>) -> Result<Vec<Vec<Entry>>, sqlx::Error> {
    let sources = sqlx::query_as::<_, Source>("SELECT DISTINCT `source` FROM `data` ORDER BY `source`")
        .fetch_all(pool).await?;
    let mut historical_data: Vec<Vec<Entry>> = Vec::new();
    for source in sources {
        let source_history = sqlx::query_as::<_, Entry>("
        SELECT * FROM (
            SELECT t.`source`, t.`time` AS time, t.`temp_c`, t.`temp_f`, t.`humidity` 
            FROM 
            (
                SELECT `source`, `time`, `temp_c`, `temp_f`, `humidity`, ROW_NUMBER() OVER (ORDER BY `time` DESC) AS row_num
                FROM `data` WHERE `source` = ? 
                ORDER BY `time`
                DESC
            ) AS t 
            WHERE t.`row_num` % 60 = 0
            ORDER BY t.`time`
            DESC
            LIMIT 168
        ) AS f
        ORDER BY f.`time`
        ASC;")
            .bind(&source.source)
            .fetch_all(pool).await?;
        let mut entry_pool: Vec<Entry> = Vec::new();
        for entry in source_history {
            let src_time = &entry.time;
            let new_time = Utc.with_ymd_and_hms(src_time.year(), src_time.month(), src_time.day(), src_time.hour(), 0, 0).unwrap();
            let new_entry = Entry {
                source: entry.source,
                time: new_time,
                temp_c: entry.temp_c,
                temp_f: entry.temp_f,
                humidity: entry.humidity,
            };
            entry_pool.push(new_entry);
        }
        historical_data.push(entry_pool);
    }
    Ok(historical_data)
}