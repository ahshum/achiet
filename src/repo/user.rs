use super::{util, Error};
use crate::{
    database::{Ops, Row, Value},
    model::User,
};

/// Manages database operations of user
pub struct UserRepo {
    table_name: String,
}

impl UserRepo {
    pub fn new() -> Self {
        Self {
            table_name: "user".to_string(),
        }
    }

    /// Lists users with filters
    pub fn list(&self) -> ListUser<'_> {
        ListUser {
            repo: self,
            filters: Vec::new(),
        }
    }

    /// Creates user with data
    pub async fn create(&self, db: &mut impl Ops, data: User) -> Result<User, Error> {
        let now = util::now();
        let user = User {
            id: util::new_id(),
            created_at: now.clone(),
            updated_at: now,
            ..data
        };

        let row = Row::try_from(user.clone()).map_err(Error::Database)?;
        let len = row.len();
        let (fields, args) = row.into_iter().map(|col| col.inner()).fold(
            (Vec::with_capacity(len), Vec::with_capacity(len)),
            |mut acc, (k, v)| {
                acc.0.push(k);
                acc.1.push(v);
                acc
            },
        );

        let mut sql = db.sql();
        sql.push("INSERT INTO ")
            .push(&self.table_name)
            .push(" (")
            .push(fields.join(", "))
            .push(") VALUES (")
            .push(vec!["?"; len].join(","))
            .bind_many(args)
            .push(")");

        db.execute(sql)
            .await
            .map_err(Error::Database)
            .and_then(util::expect_single_row(user))
    }

    /// Updates user with data
    pub async fn update(&self, db: &mut impl Ops, data: User) -> Result<User, Error> {
        let now = util::now();
        let user = User {
            updated_at: now,
            ..data
        };

        let fields = ["username", "email", "role", "updated_at"];
        let (mut stmts, mut args) = Row::try_from(user.clone())
            .map_err(Error::Database)?
            .into_iter()
            .filter(|col| fields.contains(&col.name()))
            .fold(
                (
                    Vec::with_capacity(fields.len()),
                    Vec::with_capacity(fields.len()),
                ),
                |mut acc, col| {
                    acc.0.push(format!("{0} = ?", col.name()));
                    acc.1.push(col.value().clone());
                    acc
                },
            );
        if user.password != "" {
            log::debug!("update password len:{}", user.password.len());
            stmts.push("password = ?".to_string());
            args.push(user.password.clone().into());
        }

        let mut sql = db.sql();
        sql.push("UPDATE ")
            .push(&self.table_name)
            .push(" SET ")
            .push(stmts.join(", "))
            .bind_many(args)
            .push(" WHERE id = ?")
            .bind(user.id.clone());

        db.execute(sql)
            .await
            .map_err(Error::Database)
            .and_then(util::expect_single_row(user))
    }

    /// Delete user based on the `id` in data
    pub async fn delete(&self, db: &mut impl Ops, data: User) -> Result<(), Error> {
        let mut sql = db.sql();
        sql.push("DELETE FROM ")
            .push(&self.table_name)
            .push(" WHERE id = ?")
            .bind(data.id);

        db.execute(sql)
            .await
            .map_err(Error::Database)
            .and_then(util::expect_single_row(()))
    }
}

/// User filter for searching
#[derive(Clone)]
pub enum UserFilter {
    Id(String),
    Username(String),
}

/// Lists user with filters and converts into query
pub struct ListUser<'a> {
    repo: &'a UserRepo,
    filters: Vec<UserFilter>,
}

impl ListUser<'_> {
    /// Adds filter into the query
    pub fn filter(self, filter: UserFilter) -> Self {
        let Self { repo, mut filters } = self;
        filters.push(filter);
        Self { repo, filters }
    }

    /// Runs the query of user listing
    pub async fn query(self, db: &mut impl Ops) -> Result<Vec<User>, Error> {
        let Self { repo, filters } = self;

        let len = filters.len();
        let (stmts, args) = filters
            .clone()
            .into_iter()
            .map(|f| match f {
                UserFilter::Id(id) => ("id = ?", vec![id.into()]),
                UserFilter::Username(username) => ("username = ?", vec![username.into()]),
            })
            .fold(
                (Vec::with_capacity(len), Vec::<Value>::with_capacity(len)),
                |mut acc, kv| {
                    acc.0.push(kv.0);
                    acc.1.extend(kv.1);
                    acc
                },
            );

        let mut sql = db.sql();
        sql.push("SELECT * FROM ").push(&repo.table_name);
        if len > 0 {
            sql.push(" WHERE ")
                .push(stmts.join(" AND "))
                .bind_many(args);
        }

        db.query(sql)
            .await
            .map_err(Error::Database)?
            .into_iter()
            .map(User::try_from)
            .collect::<Result<_, _>>()
            .map_err(Error::Database)
    }
}
