use std::{env, error::Error, process::exit};

use ipp::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} uri [attrs]", args[0]);
        exit(1);
    }

    let uri: Uri = args[1].parse()?;
    let client = IppClient::new(uri.clone());
    let operation = IppOperationBuilder::get_printer_attributes(uri)
        .attributes(&args[2..])
        .build();

    let attrs = client.send(operation).await?;

    for v in attrs
        .groups_of(DelimiterTag::PrinterAttributes)
        .next()
        .unwrap()
        .attributes()
        .values()
    {
        println!("{}: {}", v.name(), v.value());
    }

    Ok(())
}
