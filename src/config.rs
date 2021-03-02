use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub packages: Vec<Package>,
}