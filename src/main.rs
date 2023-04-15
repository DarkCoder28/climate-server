use std::{env, collections::HashMap};

use lazy_static::lazy_static;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use tide::{Request, prelude::*, Response};
use futures::executor;
use dotenv::dotenv;

pub mod sqlclient;
pub mod graph;
pub mod graph_html;
pub mod data_parser;

lazy_static! {
    static ref SQL_CONN: Pool<MySql> = {
        let sql_addr = match env::var("SQL_HOST") {
            Ok(x) => x,
            Err(_) => String::from("mysql"),
        };
        let sql_port = match env::var("SQL_PORT") {
            Ok(x) => x,
            Err(_) => String::from("3306"),
        };
        let sql_user = match env::var("SQL_USER") {
            Ok(x) => x,
            Err(x) => {
                println!("Error: You must specify the SQL_USER environment variable.");
                panic!("{}", x);
            },
        };
        let sql_pass = match env::var("SQL_PASS") {
            Ok(x) => x,
            Err(x) => {
                println!("Error: You must specify the SQL_PASS environment variable.");
                panic!("{}", x);
            },
        };
        let sql_db = match env::var("SQL_DB") { 
            Ok(x) => x,
            Err(x) => {
                println!("Error: You must specify the SQL_DB environment variable.");
                panic!("{}", x);
            },
        };
        let sqlserver = format!("mysql://{}:{}@{}:{}/{}", sql_user, sql_pass, sql_addr, sql_port, sql_db);
        let pool = executor::block_on(MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&sqlserver));
        let pool = match pool {
            Ok(x) => x,
            Err(x) => panic!("{}", x)
        };
        return pool;
    };
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let mut app = tide::new();
    app.with(tide_compress::CompressMiddleware::new());
    app.at("/").get(get_latest).post(push_new);
    app.at("/post").post(push_new);
    app.at("/graph.html").get(graph::serve_graph);//.serve_file("src/_dev/graph.html")?;
    app.at("/graph_data.json").get(graph::serve_graph_data);//.serve_file("src/_dev/data.json")?;

    let listen_interface = match env::var("WEB_ADDRESS") { 
        Ok(x) => x,
        Err(_) => String::from("0.0.0.0"),
    };
    let listen_port = match env::var("WEB_PORT") { 
        Ok(x) => x,
        Err(_) => String::from("3000"),
    };

    app.listen(format!("{}:{}", listen_interface, listen_port)).await?;

    Ok(())

}

async fn get_latest(_req: Request<()>) -> tide::Result {
    let latest = match sqlclient::get_latest(&SQL_CONN).await {
        Ok(x) => x,
        Err(x) => {
            println!("{}", x);
            HashMap::new()
        },
    };
    Ok(json!(latest).into())
}

async fn push_new(mut req: Request<()>) -> tide::Result {
    let parsed_data = data_parser::parse_post(&(req.body_string().await?)).await;
    let _ = match sqlclient::push_new(&SQL_CONN, parsed_data).await {
        Ok(x) => x,
        Err(x) => {
            println!("{}", x);
            ()
        },
    };
    let resp = Response::builder(200).build();
    Ok(resp)
}