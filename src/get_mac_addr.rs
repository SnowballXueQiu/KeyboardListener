use sha1::{Sha1, Digest};

pub fn get_mac_addr() -> String {
    let mac = mac_address::get_mac_address().unwrap().unwrap();
    let hash = Sha1::digest(mac.to_string().as_bytes());
    let hash = format!("{:x}", hash);
    let short_hash = &hash[0..8];

    short_hash.to_string()
}
