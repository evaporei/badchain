use crate::chain::Blockchain;
use crate::pow::ProofOfWork;
use crate::utils::hash_to_str;
use clap::{App, Arg, SubCommand};
use std::str::from_utf8;

pub fn run() -> anyhow::Result<()> {
    let matches = App::new("badchain")
        .version("1.0")
        .author("Otávio Pace <otaviopp8@gmail.com>")
        .about("Simple chain, made for learning purposes")
        .subcommand(
            SubCommand::with_name("printchain").about("Prints the whole chain (from database)"),
        )
        .subcommand(
            SubCommand::with_name("addblock")
                .about("Adds a block to the chain")
                .arg(
                    Arg::with_name("data")
                        .short("d")
                        .long("data")
                        .value_name("DATA")
                        .help("data to be added to the chain, should be string")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("printchain") {
        return printchain();
    }

    if let Some(ref matches) = matches.subcommand_matches("addblock") {
        let data = matches.value_of("data").unwrap(); // Safe because of `.required(true)`
        return addblock(data);
    }

    Ok(())
}

fn printchain() -> anyhow::Result<()> {
    let chain = Blockchain::new()?;

    for block in chain {
        let block = block?;
        println!("Prev. hash: {:?}", block.prev_block_hash);
        println!("Data: {}", from_utf8(&block.data).unwrap());
        println!("Hash: {}", hash_to_str(&block.hash));
        let pow = ProofOfWork::new(block);
        println!("PoW: {}", pow.validate());
    }

    Ok(())
}

fn addblock(data: &str) -> anyhow::Result<()> {
    let mut chain = Blockchain::new()?;
    chain.add_block(data)
}
