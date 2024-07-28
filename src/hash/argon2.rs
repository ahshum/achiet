use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2,
};

pub fn hash_password(password: String) -> Result<String, ()> {
    use argon2::password_hash::PasswordHasher;
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ())
        .map(|hashed| hashed.to_string())
}

pub fn verify_password(password: String, hashed_password: String) -> Result<(), ()> {
    use argon2::password_hash::PasswordVerifier;
    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ())
        .map(|_| ())
}
