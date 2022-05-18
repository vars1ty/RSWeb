#![allow(nonstandard_style)] // <= To make Rust compiler stop complaining about not using snake_case.

use std::error::Error;

use serde::Deserialize;

// * Variables.
const URL_STRING: &str = "https://v2.api.iphub.info/guest/ip/";

/// Startup function.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // * Cache the response so we don't have to make a request every time.
    let info = get_info("9.9.9.9").await?;
    println!("IP: {}", info.ip);
    println!("Country Code: {}", info.countryCode);
    println!("Country: {}", info.countryName);
    println!("ISP: {}", info.isp);
    println!("Blocked: {}", info.block);
    Ok(())
}

/// IPHub JSON Keys and their corresponding Rust types.
#[derive(Deserialize)]
pub struct IPHub {
    pub ip: String,
    pub countryCode: String,
    pub countryName: String,
    pub asn: i32,
    pub isp: String,
    pub block: i32,
    pub hostname: String,
}

/// Get the IP information from IPHub.
async fn get_info(ip: &str) -> Result<IPHub, Box<dyn Error>> {
    let resp: IPHub = reqwest::get(URL_STRING.to_owned() + ip).await?.json::<IPHub>().await?;
    Ok(resp)
}