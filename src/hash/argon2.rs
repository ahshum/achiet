use argon2::password_hash::{self, rand_core};

pub struct Argon2<'a>(argon2::Argon2<'a>);

impl Argon2<'_> {
    pub fn new() -> Self {
        Self(argon2::Argon2::default())
    }

    pub fn hash(&self, raw: &str) -> Result<String, password_hash::Error> {
        use argon2::PasswordHasher;

        let salt = password_hash::SaltString::generate(&mut rand_core::OsRng);
        self.0
            .hash_password(raw.as_bytes(), &salt)
            .map(|h| h.to_string())
    }

    pub fn verify(&self, raw: &str, hash: &str) -> Result<(), password_hash::Error> {
        use argon2::PasswordVerifier;

        let hpw = argon2::PasswordHash::new(hash)?;
        self.0.verify_password(raw.as_bytes(), &hpw)
    }
}
