use warp::Filter;
use serde::{Deserialize, Serialize};
use rand::Rng;
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM encryption
use aes_gcm::aead::{Aead, NewAead};

#[derive(Serialize, Deserialize)]
struct WalletRequest {
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct WalletResponse {
    public_key: String,
    encrypted_private_key: String,
}

// Simulate key generation
fn generate_keys() -> (String, String) {
    let mut rng = rand::thread_rng();
    let public_key = format!("PUB-{:x}", rng.gen::<u64>());
    let private_key = format!("PRIV-{:x}", rng.gen::<u64>());
    (public_key, private_key)
}

// Simulate post-quantum encryption of private key
fn encrypt_private_key(private_key: &str) -> String {
    let key = Key::from_slice(&[0u8; 32]);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&[0u8; 12]);
    let ciphertext = cipher.encrypt(nonce, private_key.as_bytes()).unwrap();
    hex::encode(ciphertext)
}

#[tokio::main]
async fn main() {
    // POST /create_wallet
    let create_wallet = warp::post()
        .and(warp::path("create_wallet"))
        .and(warp::body::json())
        .map(|req: WalletRequest| {
            let (pub_key, priv_key) = generate_keys();
            let encrypted_priv = encrypt_private_key(&priv_key);
            warp::reply::json(&WalletResponse {
                public_key: pub_key,
                encrypted_private_key: encrypted_priv,
            })
        });

    println!("Quantum Wallet backend running");
    warp::serve(create_wallet).run(([127, 0, 0, 1], 3030)).await;
}
