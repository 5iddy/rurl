#![allow(unused)]


mod cliargs;
mod request;
use cliargs::{CliArgs,Method, parse_headers_to_headermap};
use clap::Parser;
use reqwest::header::HeaderMap;

use std::path::PathBuf;
use std::process::exit;
use std::collections::HashMap;
use std::str::FromStr;
use serde_json::Value;

use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

use pbr::{ProgressBar, Units};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CliArgs = CliArgs::parse();

    let client = reqwest::Client::new();

    let output_filepath = match args.out {
        Some(filename) => filename,
        None => PathBuf::from_str(args.url.path().split("/").last().unwrap_or("output.txt")).unwrap()
    };

    let method = match args.method {
        Some(method) => method,
        None => Method::GET
    };

    let headers_map = match args.headers{
        Some(headers) => parse_headers_to_headermap(&headers),
        None => HeaderMap::new()
    };

    // let headers_map = parse_headers_to_headermap(&headers);

    let mut response = match method {
        Method::GET => {
            client.get(args.url).headers(headers_map).send().await?
        },
        Method::POST => {
            client.post(args.url).headers(headers_map).body(args.data.unwrap_or_default()).send().await?
        },
        _ => {
            client.get(args.url).send().await?
        }
    };

    if args.download {
        let count = response.content_length().unwrap();
        let mut pb = ProgressBar::new(count);
        pb.set_units(Units::Bytes);
        let message = format!("Downloading {}\n", output_filepath.to_str().unwrap().to_owned());
        pb.message(&message);
        let mut outfile = File::create(output_filepath).await?;
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let n = outfile.write(&chunk.expect("Failed to get chunck")).await.expect("Failed to write to file");
            pb.add(n.try_into().expect("Failed to convert usize into u64"));
        }
        pb.finish();
    }
    else {
        let body = response.text().await?;
        println!("{}", body);
    }
    Ok(())
}


