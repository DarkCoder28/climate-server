use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use crate::sqlclient::{self, Entry};

#[derive(Serialize, Deserialize)]
struct PostDataPoint {
    temp: Decimal,
    humid: Option<f32>,
}

pub async fn parse_post(data: &str) -> Vec<sqlclient::Entry> {
    let mut parsed_data = Vec::new();
    let data = String::from(data);
    let data = Vec::from_iter(data.split("&").map(|x| String::from(x)));
    for source_data in data {
        let temp = Vec::from_iter(source_data.split("=").map(|x| String::from(x)));
        let source = match temp.get(0) {
            Some(x) => x.to_owned(),
            None => continue,
        };
        let json_data = match temp.get(1) {
            Some(x) => x.to_owned(),
            None => continue,
        };
        let parsed_json: PostDataPoint = match serde_json::from_str::<PostDataPoint>(&json_data) {
            Ok(x) => x,
            Err(x) => {
                println!("{}", x);
                continue
            },
        };
        let temp_f = (&parsed_json.temp * dec!(1.8)) + dec!(32);
        let new_entry = Entry {
            source: source,
            time: chrono::Utc::now(),
            temp_c: parsed_json.temp,
            temp_f: temp_f,
            humidity: parsed_json.humid,
        };
        parsed_data.push(new_entry);
    }
    parsed_data
}