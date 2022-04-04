use clap::{
    Parser, 
    Arg,
    ArgEnum
};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName, self};
use url::Url;
use core::fmt;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::error::Error;
// use clap::PossibleValue;


/// A modern alternative to curl built in rust
#[derive(Parser, Debug)]
#[clap(version)] 
pub struct CliArgs {
    /// Http method to use 
    #[clap(short='X', long)]
    #[clap(arg_enum)]
    pub method: Option<Method>,

    /// Output response to file with -o <filename>
    #[clap(short='o', long, value_name="filename")]
    pub out: Option<PathBuf>, 

    /// Download the file at <URL>
    /// use -o option to select the filename
    /// or the filename is inferred from the url
    #[clap(long)]
    pub download: bool,

    /// Add headers with -H, you can add multiple headers
    #[clap(short='H', long)]
    pub headers: Option<Vec<String>>,

    /// Add body to the request
    #[clap(short='d', long)]
    pub data: Option<String>,

    /// Add query parameters using -q
    #[clap(short='q')]
    pub query_params: Option<Vec<String>>,

    /// save cookies to <filename>
    #[clap(short='c', long)]
    pub save_cookies_to: Option<PathBuf>,

    /// load cookies with -b <key>=<value>
    #[clap(short='b', long)]
    pub cookies: Option<Vec<String>>,

    /// Url to send the request to
    pub url: Url
}

#[derive(Clone, Debug, ArgEnum)]
#[clap(rename_all="UPPER")]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    PATCH,
    TRACE
}

pub struct HeaderParsingError;

impl Display for HeaderParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured while parsing Headers.")
    }
}


/// Parse a key-value strings into a hashmap
pub fn parse_headers_to_headermap<'a>(headers: &'a Vec<String>) -> HeaderMap{
    // let mut headers_map = HeaderMap::new();

    let mut map = HashMap::<String, String>::new();

    for header in headers {
        let pair:Vec<&str> = header.split(":").collect();
        let key = pair[0];
        let value = pair[1].trim_start();
        map.insert(key.to_string(), value.to_string());
    }

    let headers_map:HeaderMap = (&map).try_into().expect("Not valid headers");

    headers_map
}