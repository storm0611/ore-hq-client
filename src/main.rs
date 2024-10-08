use claim::ClaimArgs;
use solana_sdk::signature::{read_keypair_file, Keypair};
use clap::{Parser, Subcommand};

use signup::signup;

mod signup;
mod protomine;
mod mine;
mod claim;
mod balance;
mod rewards;

// --------------------------------

/// A command line interface tool for pooling power to submit hashes for proportional ORE rewards
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Args {
    #[arg(long,
        value_name = "SERVER_URL",
        help = "URL of the server to connect to",
        default_value = "ec1ipse.me",
    )]
    url: String,

    #[arg(
        long,
        value_name = "KEYPAIR_PATH",
        help = "Filepath to keypair to use",
        default_value = "~/.config/solana/id.json",
    )]
    keypair: String,

    #[arg(
        long,
        short,
        action,
        help = "Use unsecure http connection instead of https.",
    )]
    use_http: bool,

    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Connect to pool and start mining. (Default Implementation)")]
    Mine(mine::MineArgs),
    #[command(about = "Connect to pool and start mining. (Protomine Implementation)")]
    Protomine(protomine::MineArgs),
    #[command(about = "Transfer sol to the pool authority to sign up.")]
    Signup,
    #[command(about = "Claim rewards.")]
    Claim(ClaimArgs),
    #[command(about = "Display claimable rewards.")]
    Rewards,
    #[command(about = "Display current ore token balance.")]
    Balance,
}

// --------------------------------


#[tokio::main]
async fn main() {
    let args = Args::parse();

    let base_url = args.url;
    let unsecure_conn = args.use_http;
    let keypair = args.keypair.clone();
    match args.command {
        Commands::Mine(args) => {
            let key = read_keypair_file(keypair.clone()).expect(&format!("Failed to load keypair from file: {}", keypair));
            mine::mine(args, key, base_url, unsecure_conn).await;
        },
        Commands::Protomine(args) => {
            protomine::mine(args, key, base_url, unsecure_conn).await;
        },
        Commands::Signup => {
            let key = Keypair::from_base58_string(keypair.as_str());
            signup(base_url, key, unsecure_conn).await;
        },
        Commands::Claim(args) => {
            let key = read_keypair_file(keypair.clone()).expect(&format!("Failed to load keypair from file: {}", keypair));
            claim::claim(args, key, base_url, unsecure_conn).await;
        }
        Commands::Rewards => {
            let key = read_keypair_file(keypair.clone()).expect(&format!("Failed to load keypair from file: {}", keypair));
            rewards::rewards(key, base_url, unsecure_conn).await;
        }
        Commands::Balance => {
            let key = read_keypair_file(keypair.clone()).expect(&format!("Failed to load keypair from file: {}", keypair));
            balance::balance(key, base_url, unsecure_conn).await;
        }
    }


}

