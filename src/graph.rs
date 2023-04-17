use serde::Serialize;
use tide::{Request, Response, http::mime, prelude::*};
use chrono::{prelude::*};
use lazy_static::lazy_static;
use rust_decimal::Decimal;
use crate::{sqlclient, SQL_CONN, graph_html};

pub async fn serve_graph(_req: Request<()>) -> tide::Result {
    let resp = Response::builder(200)
        .body(graph_html::GRAPH_HTML.as_str())
        .content_type(mime::HTML)
        .build();
    Ok(resp)
}

#[derive(Serialize)]
struct TempHumidData {
    temp: TempData,
    humid: HumidData,
}

#[derive(Serialize)]
struct TempData {
    labels: Vec<String>,
    datasets: Vec<TempDataSet>,
}

#[derive(Serialize)]
struct TempDataSet {
    label: String,
    data: Vec<Decimal>,
    #[serde(rename(serialize = "borderColor", deserialize = "borderColor"))]
    border_color: String,
    #[serde(rename(serialize = "backgroundColor", deserialize = "backgroundColor"))]
    background_color: String,
}

#[derive(Serialize)]
struct HumidData {
    labels: Vec<String>,
    datasets: Vec<HumidDataSet>,
}

#[derive(Serialize)]
struct HumidDataSet {
    label: String,
    data: Vec<f32>,
    #[serde(rename(serialize = "borderColor", deserialize = "borderColor"))]
    border_color: String,
    #[serde(rename(serialize = "backgroundColor", deserialize = "backgroundColor"))]
    background_color: String,
}


lazy_static! {
    static ref COLOURS: Vec<String> = Vec::from_iter(
        String::from("rgb(255, 99, 132)|rgb(255, 159, 64)|rgb(255, 205, 86)|rgb(75, 192, 192)|rgb(54, 162, 235)|rgb(153, 102, 255)|rgb(201, 203, 207)")
        .split("|")
        .map(|x| String::from(x))
    );
    static ref COLOURS_TRANSPARENT: Vec<String> = Vec::from_iter(
        String::from("rgba(255, 99, 132, 0.5)|rgba(255, 159, 64, 0.5)|rgba(255, 205, 86, 0.5)|rgba(75, 192, 192, 0.5)|rgba(54, 162, 235, 0.5)|rgba(153, 102, 255, 0.5)|rgba(201, 203, 207, 0.5)")
        .split("|")
        .map(|x| String::from(x))
    );
}
fn get_border_colour(index: &usize) -> String {
    let index = index % COLOURS.len();
    match COLOURS.get(index.clone()) {
        Some(x) => x.to_owned(),
        None => String::from("rgb(255,99,132)"),
    }
}
fn get_background_colour(index: &usize) -> String {
    let index = index % COLOURS_TRANSPARENT.len();
    match COLOURS_TRANSPARENT.get(index.clone()) {
        Some(x) => x.to_owned(),
        None => String::from("rgba(255,99,132,.5)"),
    }
}

pub async fn serve_graph_data(_req: Request<()>) -> tide::Result {
    let graph_data = match sqlclient::get_history(&SQL_CONN).await {
        Ok(x) => x,
        Err(x) => {
            println!("{}", x);
            panic!("{}", x);
        },
    };
    let mut have_labels:bool = false;
    let mut labels: Vec<String> = Vec::new();
    let mut temp_datasets: Vec<TempDataSet> = Vec::new();
    let mut humid_datasets: Vec<HumidDataSet> = Vec::new();
    let mut index: usize = 0;
    for data in graph_data {
        let mut new_temp_data = Vec::new();
        let mut new_humid_data = Vec::new();
        let mut have_src = false;
        let mut source = String::new();
        for entry in data {
            if !have_src {
                source = String::from(&entry.source);
                have_src = true;
            }
            if !have_labels {
                let time = (&entry.time).to_owned().with_timezone::<Local>(&chrono::Local).format("%m-%d-%Y %H").to_string();
                labels.push(time);
            }
            new_temp_data.push((&entry.temp_f).to_owned());
            match entry.humidity {
                Some(x) => {
                    new_humid_data.push(x);
                },
                None => (),
            }
        }
        let temp_dataset = TempDataSet {
            label: (&source).to_owned(), 
            data: new_temp_data,
            border_color: get_border_colour(&index),
            background_color: get_background_colour(&index),
        };
        if !(&new_humid_data).is_empty() {
            let humid_dataset = HumidDataSet {
                label: (&source).to_owned(), 
                data: new_humid_data,
                border_color: get_border_colour(&index),
                background_color: get_background_colour(&index),
            };
            humid_datasets.push(humid_dataset);
        }
        temp_datasets.push(temp_dataset);
        index += 1;
        have_labels = true;
    }
    let temp_data = TempData {
        labels: (&labels).to_owned(),
        datasets: temp_datasets,
    };
    let humid_data = HumidData {
        labels: (&labels).to_owned(),
        datasets: humid_datasets,
    };
    let response_data = TempHumidData {
        temp: temp_data,
        humid: humid_data,
    };
    Ok(json!(response_data).into())
}