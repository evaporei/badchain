use crate::utils::sha256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,
    inputs: Vec<TrxInput>,
    outputs: Vec<TrxOutput>,
}

// amount of the reward, it will be dynamic in the future
const SUBSIDY: usize = 10;

impl Transaction {
    pub fn new_coinbase(to: &str, data: &str) -> Self {
        let data = match data.is_empty() {
            true => format!("Reward to '{}'", to),
            false => data.to_owned(),
        };

        let trx_in = TrxInput {
            trx_id: vec![],
            output_idx: -1, // TODO: stop using -1 to represent state, this isn't C
            script_sig: data,
        };
        let trx_out = TrxOutput {
            value: SUBSIDY,
            script_pub_key: to.to_owned(),
        };

        let mut trx = Self {
            id: vec![],
            inputs: vec![trx_in],
            outputs: vec![trx_out],
        };

        trx.set_id();

        trx
    }

    // TODO: maybe kill this method
    fn set_id(&mut self) {
        let encoded_trx = bincode::serialize(&self).unwrap();
        let hash = sha256(&encoded_trx);
        self.id = hash;
    }
}

#[derive(Serialize, Deserialize)]
pub struct TrxInput {
    trx_id: Vec<u8>,
    output_idx: isize,
    script_sig: String,
}

#[derive(Serialize, Deserialize)]
pub struct TrxOutput {
    value: usize,
    script_pub_key: String,
}
