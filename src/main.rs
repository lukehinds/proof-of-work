use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

const DIFFICULTY: &str = "99";

fn main() {
    let genesis_hash = "0000000000000000000000000000000000000000000000000000000000000000";

    let mut nonce = 0;
    loop {
        let mut hasher = Sha256::new();
        let concated_string = format!(
            "{}{}{}",
            genesis_hash,
            nonce.to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        hasher.update(&concated_string);

        let hash = hasher.finalize();
        let hash_string = format!("{:x}", hash);

        if hash_string.starts_with(DIFFICULTY) {
            println!("solved with hash: {}, using nonce {}", hash_string, nonce);
            break;
        }
        nonce += 1;
    }
}
