use argon2::{self, Config, verify_encoded};

pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let password = password.as_bytes();
    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    Ok(hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let matches = verify_encoded(&password_hash, password.as_bytes()).unwrap();

    Ok(matches)
}
