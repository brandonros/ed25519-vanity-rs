mod solana;
mod bitcoin;
mod shallenge;
mod ethereum;

use std::error::Error;
use std::sync::Arc;

use common::GlobalStats;

#[derive(Debug, Clone)]
enum Mode {
    SolanaVanity { prefix: String, suffix: String },
    BitcoinVanity { prefix: String, suffix: String },
    EthereumVanity { prefix: String, suffix: String },
    Shallenge { username: String, target_hash: String },
}

fn cpu_main(
    num_threads: usize,
    mode: Mode,
    global_stats: Arc<GlobalStats>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match mode {
        Mode::SolanaVanity { prefix, suffix } => {
            solana::cpu_main_solana_vanity(num_threads, prefix, suffix, global_stats)
        }
        Mode::BitcoinVanity { prefix, suffix } => {
            bitcoin::cpu_main_bitcoin_vanity(num_threads, prefix, suffix, global_stats)
        }
        Mode::EthereumVanity { prefix, suffix } => {
            ethereum::cpu_main_ethereum_vanity(num_threads, prefix, suffix, global_stats)
        }
        Mode::Shallenge { username, target_hash } => {
            let target_hash_bytes = hex::decode(target_hash)?;
            shallenge::cpu_main_shallenge(num_threads, username, target_hash_bytes, global_stats)
        }
    }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = std::env::args().collect::<Vec<String>>();
    
    let mode = if args.len() == 4 && args[1] == "solana-vanity" {
        let vanity_prefix = args[2].clone();
        let vanity_suffix = args[3].clone();
        if vanity_prefix.len() > 0 {
            common::validate_base58_string(&vanity_prefix)?;
        }
        if vanity_suffix.len() > 0 {
            common::validate_base58_string(&vanity_suffix)?;
        }
        Mode::SolanaVanity { prefix: vanity_prefix, suffix: vanity_suffix }
    } else if args.len() == 4 && args[1] == "bitcoin-vanity" {
        let vanity_prefix = args[2].clone();
        let vanity_suffix = args[3].clone();
        if vanity_prefix.len() > 0 {
            common::validate_bech32_string(&vanity_prefix)?;
        }
        if vanity_suffix.len() > 0 {
            common::validate_bech32_string(&vanity_suffix)?;
        }
        Mode::BitcoinVanity { prefix: vanity_prefix, suffix: vanity_suffix }
    } else if args.len() == 4 && args[1] == "ethereum-vanity" {
        let vanity_prefix = args[2].clone();
        let vanity_suffix = args[3].clone();
        if vanity_prefix.len() > 0 {
            common::validate_hex_string(&vanity_prefix)?;
        }
        if vanity_suffix.len() > 0 {
            common::validate_hex_string(&vanity_suffix)?;
        }
        Mode::EthereumVanity { prefix: vanity_prefix, suffix: vanity_suffix }
    } else if args.len() == 4 && args[1] == "shallenge" {
        let username = args[2].clone();
        let target_hash = args[3].clone();
        common::validate_hex_string(&target_hash)?;
        Mode::Shallenge { username, target_hash }
    } else {
        println!("Usage:");
        println!("  {} solana-vanity <prefix> <suffix>", args[0]);
        println!("  {} bitcoin-vanity <prefix> <suffix>", args[0]);        
        println!("  {} ethereum-vanity <prefix> <suffix>", args[0]);
        println!("  {} shallenge <username> <target_hash_hex>", args[0]);
        std::process::exit(1);
    };

    let num_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);

    let global_stats = match &mode {
        Mode::SolanaVanity { prefix, suffix } => Arc::new(GlobalStats::new(
            num_threads,
            prefix.len(),
            suffix.len()
        )),
        Mode::BitcoinVanity { prefix, suffix } => Arc::new(GlobalStats::new(
            num_threads,
            prefix.len(),
            suffix.len()
        )),
        Mode::EthereumVanity { prefix, suffix } => Arc::new(GlobalStats::new(
            num_threads,
            prefix.len(),
            suffix.len()
        )),
        Mode::Shallenge { username, .. } => Arc::new(GlobalStats::new(
            num_threads,
            username.len(),
            0 // No suffix for shallenge
        )),
    };

    match &mode {
        Mode::SolanaVanity { prefix, suffix } => {
            println!("Searching for solana vanity key with prefix '{}' and suffix '{}'", prefix, suffix);
        }
        Mode::BitcoinVanity { prefix, suffix } => {
            println!("Searching for bitcoin vanity key with prefix '{}' and suffix '{}'", prefix, suffix);
        }
        Mode::EthereumVanity { prefix, suffix } => {
            println!("Searching for ethereum vanity key with prefix '{}' and suffix '{}'", prefix, suffix);
        }
        Mode::Shallenge { username, target_hash } => {
            println!("Starting shallenge for username '{}' with target hash '{}'", username, target_hash);
        }
    }

    // Launch appropriate compute mode
    cpu_main(num_threads, mode, global_stats)
}
