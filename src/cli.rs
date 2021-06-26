use crate::chain;
use crate::chain::Blockchain;
use crate::chain::DATABASE_FILE_NAME;
use crate::pow::ProofOfWork;
use crate::utils::hash_to_str;
use clap::{App, Arg, SubCommand};
use std::path::Path;

pub fn run() -> anyhow::Result<()> {
    let matches = App::new("badchain")
        .version("1.0")
        .author("Otávio Pace <otaviopp8@gmail.com>")
        .about("Simple chain, made for learning purposes")
        .subcommand(
            SubCommand::with_name("createchain")
                .about("Creates the blockchain")
                .arg(
                    Arg::with_name("address")
                        .long("address")
                        .value_name("ADDRESS")
                        .help("address which will receive the reward for mining the genesis block, should be string")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("printchain").about("Prints the whole chain (from database)"),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("createchain") {
        let address = matches.value_of("address").unwrap(); // Safe because of `.required(true)`
        return createchain(address);
    }

    if let Some(_) = matches.subcommand_matches("printchain") {
        return printchain();
    }

    Ok(())
}

fn db_exists() -> bool {
    Path::new(DATABASE_FILE_NAME).is_dir()
}

fn createchain(address: &str) -> anyhow::Result<()> {
    if db_exists() {
        println!("Blockchain already exists.");
        std::process::exit(1);
    }

    chain::setup_database(address)?;

    println!("Done!");

    Ok(())
}

fn printchain() -> anyhow::Result<()> {
    let chain = Blockchain::new()?;

    for block in chain {
        let block = block?;
        println!("Prev. hash: {:?}", block.prev_block_hash);
        println!("Hash: {}", hash_to_str(&block.hash));
        let pow = ProofOfWork::new(block);
        println!("PoW: {}", pow.validate());
    }

    Ok(())
}
