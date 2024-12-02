use super::Error;
use crate::{
    database::Ops,
    field,
    model::User,
    repo::{UserFilter, UserRepo},
};

pub struct UserSvc {
    user: UserRepo,
}

impl UserSvc {
    pub fn new() -> Self {
        Self {
            user: UserRepo::new(),
        }
    }

    pub async fn list(&self, db: &mut impl Ops) -> Result<Vec<User>, Error> {
        self.user.list().query(db).await.map_err(Error::Repo)
    }

    pub async fn find_by_id(&self, db: &mut impl Ops, id: field::Id) -> Result<User, Error> {
        self.user
            .list()
            .filter(UserFilter::Id(id))
            .query(db)
            .await
            .map_err(Error::Repo)?
            .into_iter()
            .next()
            .ok_or(Error::NotFound)
    }

    pub async fn find_by_username(
        &self,
        db: &mut impl Ops,
        username: String,
    ) -> Result<User, Error> {
        self.user
            .list()
            .filter(UserFilter::Username(username))
            .query(db)
            .await
            .map_err(Error::Repo)?
            .into_iter()
            .next()
            .ok_or(Error::NotFound)
    }

    pub async fn create(&self, db: &mut impl Ops, data: User) -> Result<User, Error> {
        let data = User {
            password: self.hash_password(data.password.clone().as_str())?,
            ..data
        };
        self.user.create(db, data).await.map_err(Error::Repo)
    }

    pub async fn update(&self, db: &mut impl Ops, data: User) -> Result<User, Error> {
        let data = User {
            password: if !data.password.is_empty() {
                self.hash_password(data.password.clone().as_str())?
            } else {
                data.password
            },
            ..data
        };
        self.user.update(db, data).await.map_err(Error::Repo)
    }

    pub async fn delete(&self, db: &mut impl Ops, data: User) -> Result<(), Error> {
        self.user.delete(db, data).await.map_err(Error::Repo)
    }

    pub fn hash_password(&self, plain: &str) -> Result<String, Error> {
        use argon2::PasswordHasher;

        let salt = argon2::password_hash::SaltString::generate(&mut rand_core::OsRng);
        argon2::Argon2::default()
            .hash_password(plain.as_bytes(), &salt)
            .map_err(Error::PasswordHash)
            .map(|h| h.to_string())
    }

    pub fn verify_password(&self, plain: &str, hash_str: &str) -> Result<(), Error> {
        use argon2::PasswordVerifier;

        let hash_pw = argon2::PasswordHash::new(hash_str).map_err(Error::PasswordHash)?;
        argon2::Argon2::default()
            .verify_password(plain.as_bytes(), &hash_pw)
            .map_err(Error::PasswordHash)
    }

    pub fn jwt_encode<T: serde::Serialize>(
        &self,
        claims: &T,
        secret: &str,
    ) -> Result<String, Error> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|_| Error::Jwt)
    }

    pub fn jwt_decode<T: serde::de::DeserializeOwned>(
        &self,
        token: &str,
        secret: &str,
    ) -> Result<T, Error> {
        jsonwebtoken::decode::<T>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Header::default().alg),
        )
        .map_err(|_| Error::Jwt)
        .map(|d| d.claims)
    }
}
