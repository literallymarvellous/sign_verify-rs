use std::str::FromStr;

use ethers::{signers::{Signer, LocalWallet, Wallet}, 
prelude::{SignerMiddleware, k256::ecdsa::SigningKey}, 
providers::{Provider, Http, Middleware}, types::{H160, Signature, SignatureError}};

pub async fn setup_signer(priv_key: &str, http_provider: &str) -> Option<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
  let provider = Provider::<Http>::try_from(
    http_provider
  ).expect("could not instantiate HTTP Provider");
  let chain_id: u64 = provider.get_chainid().await.unwrap().as_u64();

  let wallet = priv_key.parse::<LocalWallet>();

  match wallet {
      Ok(x) => {
          let w = x.with_chain_id(chain_id).clone();
          return Some(SignerMiddleware::new(provider, w));
      }
      Err(_) => {
          println!("Failed to connect to wallet.");
          return None;
      }
  }
}

pub async fn sign_message(signer: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>, message: &str) -> Option<(Signature, String)> {
    match signer.signer().sign_message(message).await {
        Ok(sig) => {
                println!("Produced signature {} with {}.", sig, signer.address());
                
                // verify the signature produced from your wallet.
                sig.verify(message, signer.address()).unwrap();
                println!("Verified signature produced by {:?}!", signer.address());

                return Some((sig, message.to_string()))
            },
            Err(_) => {
                return None;
            }
      }

}

pub fn recover_sig(signature: Signature, message: &str) -> Option<H160> {
  match signature.recover(message) {
    Ok(signer) => return Some(signer),
    Err(_) => return None,
  }
}

pub fn verify_sig_str(signature: &str, msg: &str) -> Option<H160> {
    let sig: Signature = FromStr::from_str(signature).unwrap();
    println!("{:?}", sig);
    let r: Result<H160, SignatureError> = sig.recover(msg);
    match r {
        Ok(signer) => return Some(signer),
        Err(_) => return None,
    }
}