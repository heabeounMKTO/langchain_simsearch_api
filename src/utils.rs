use anyhow::{Error, Result};
use deadpool_postgres::{GenericClient, Manager, Pool};
use dotenvy::dotenv;
use postgres::NoTls;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Debug)]
pub struct StringEmbeddingPair {
    pub raw_string: String,
    pub embedding: Option<Vec<f32>>,
    pub similarity: f64,
}

/// bro stole the idea
/// from raytracing engine smh
pub trait Interval {
    /// checks to see if the number provided is within the min/max bounds
    fn check_bounds(&self, min: f64, max: f64) -> bool;
}

pub async fn new_pg_pool() -> Result<Pool> {
    dotenv().ok();
    let DB_HOST = env::var("DB_HOST").expect("cannot read DB_HOST");
    let DB_PORT = env::var("DB_PORT").expect("cannot read DB_PORT");
    let DB_USER = env::var("DB_USER").expect("cannot read DB_USER");
    let DB_PASSWORD = env::var("DB_PASSWORD").expect("cannot read DB_PASSWORD");
    let DB_DATABASE = env::var("DB_DATABASE").expect("cannot read DB_DATABASE");
    let mut poolcfg = deadpool_postgres::Config::default();
    poolcfg.user = Some(DB_USER);
    poolcfg.password = Some(DB_PASSWORD);
    poolcfg.host = Some(DB_HOST);
    poolcfg.dbname = Some(DB_DATABASE);
    poolcfg.port = Some(DB_PORT.parse::<u16>().unwrap());
    let pool: Pool = poolcfg.create_pool(None, NoTls).unwrap();
    Ok(pool)
}
