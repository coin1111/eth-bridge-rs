use ethers::{
    providers::{Http, Provider},
    signers::Wallet,
    types::TransactionRequest,
};
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::iter::FromIterator;
use std::str::FromStr;
pub fn get_signer(
    signers: &HashMap<&str, ethers::signers::Wallet>,
    nick: &str,
) -> Result<ethers::signers::Wallet, String> {
    if (!nick.clone().starts_with("0x")) {
             match signers.get(&nick as &str)  {
            Some(w) => Ok(w.clone()),
            None => Err(format!("user {} is not found",nick))
        }
    } else {
        ethers::signers::Wallet::from_str(&nick).map_err(|e|{e.to_string()})
    }
}

fn get_private_key(path: &str) -> Result<String, String> {
    let line = fs::read_to_string(path).map_err(|e| e.to_string())?;
    // line[1].split(":")[1].trim();
    match line.split("\n").take(2).last().and_then(|s| {
        s.clone()
            .split(":")
            .take(2)
            .last()
            .and_then(|s| Some(s.clone().trim()))
    }) {
        Some(s) => Ok(String::from(s)),
        None => Err(format!("can't find private key: {}", path)),
    }
}
pub fn get_signers() -> Result<HashMap<&'static str, ethers::signers::Wallet>, String> {
    let names = vec!["alice", "bob", "carol", "pete", "todd", "bridgeEscrow"];
    let mut pairs: Vec<(&str, ethers::signers::Wallet)> = Vec::new();
    for n in names.iter() {
        let key = get_private_key(&format!("accounts/{}.txt", n))?;
        let w = ethers::signers::Wallet::from_str(&key).map_err(|e| e.to_string())?;
        pairs.push((n, w));
    }

    let hm: HashMap<&str, ethers::signers::Wallet> = pairs.iter().cloned().collect();
    Ok(hm)
}
