use std::env;
use dotenv::dotenv;

use web3_auth_rs::sign_in::{sign_message, setup_signer, recover_sig};


#[tokio::main]
async fn main() {
    dotenv().ok();

    let provider = "https://mainnet.infura.io/v3/e5f57e3684bf43068b71f7a25887cd46";
    let key = env::var("PRIVATE_KEY").expect("No PRIVATE_KEY environment variable");

    let signer = setup_signer(&key, provider).await.unwrap();

    let message = "marvel";
    let(sig, msg) = sign_message(signer, message).await.unwrap();

    let address = recover_sig(sig, &msg).unwrap();
    println!("public address {}", address);
}
