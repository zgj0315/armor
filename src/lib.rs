use chrono::Local;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};
use uuid::Uuid;

fn make_aes_key() -> String {
    let uuid = Uuid::new_v4();
    let timestamp = Local::now().timestamp_millis();
    let mut rng = thread_rng();
    let random = rng.gen_range(0..=999999);
    let key = format!("{}{}{:06}", uuid, timestamp, random);
    hex::encode(Sha256::digest(key))
}

#[cfg(test)]
mod tests {
    use dev_util::log::log_init;

    use super::*;

    // cargo test tests::test_make_aes_key
    #[test]
    fn test_make_aes_key() {
        log_init();
        for _ in 0..1000 {
            let key = make_aes_key();
            log::info!("key: {}", key);
        }
    }
}
