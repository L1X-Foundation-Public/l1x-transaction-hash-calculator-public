mod transaction;
mod transaction_json;
mod types;

use clap::{command, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    json: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let tx = transaction_json::TransactionJson::from_json_str(&args.json)?;
    println!("{}", hex::encode(tx.transaction_hash()?));

    Ok(())
}
