use super::{util, Error};
use crate::{
    database::{Ops, Row, Value},
    model::Bookmark,
};

/// Manages database operations of bookmark
pub struct BookmarkRepo {
    table_name: String,
}

impl BookmarkRepo {
    pub fn new() -> Self {
        Self {
            table_name: "bookmark".to_string(),
        }
    }

    /// Lists bookmarks with filters
    pub fn list(&self) -> ListBookmark<'_> {
        ListBookmark {
            repo: self,
            filters: Vec::new(),
        }
    }

    /// Creates bookmark with data
    pub async fn create(&self, db: &mut impl Ops, data: Bookmark) -> Result<Bookmark, Error> {
        let now = util::now();
        let bookmark = Bookmark {
            id: util::new_id(),
            created_at: now.clone(),
            updated_at: now,
            ..data
        };

        let row = Row::try_from(bookmark.clone()).map_err(Error::Database)?;
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
            .push(vec!["?"; len].join(", "))
            .bind_many(args)
            .push(")");

        db.execute(sql)
            .await
            .map_err(Error::Database)
            .and_then(util::expect_single_row(bookmark))
    }

    /// Updates bookmark with data
    pub async fn update(&self, db: &mut impl Ops, data: Bookmark) -> Result<Bookmark, Error> {
        let now = util::now();
        let bookmark = Bookmark {
            updated_at: now,
            ..data
        };

        let fields = ["title", "url", "description", "user_id", "updated_at"];
        let (stmts, args) = Row::try_from(bookmark.clone())
            .map_err(Error::Database)?
            .into_iter()
            .filter(|col| fields.contains(&col.name()))
            .fold(
                (
                    Vec::with_capacity(fields.len()),
                    Vec::with_capacity(fields.len()),
                ),
                |mut acc, col| {
                    acc.0.push(format!("{0} = COALESCE(?, {0})", col.name()));
                    acc.1.push(col.value().clone());
                    acc
                },
            );

        let mut sql = db.sql();
        sql.push("UPDATE ")
            .push(&self.table_name)
            .push(" SET ")
            .push(stmts.join(", "))
            .bind_many(args)
            .push(" WHERE id = ?")
            .bind(bookmark.id.clone());

        db.execute(sql)
            .await
            .map_err(Error::Database)
            .and_then(util::expect_single_row(bookmark))
    }

    /// Delete bookmark based on the `id` in data
    pub async fn delete(&self, db: &mut impl Ops, data: Bookmark) -> Result<(), Error> {
        let mut sql = db.sql();
        sql.push("DELETE FROM ")
            .push(&self.table_name)
            .push(" WHERE id = ?")
            .bind(data.id.clone());

        db.execute(sql)
            .await
            .map_err(Error::Database)
            .and_then(util::expect_single_row(()))
    }
}

/// Bookmark filter for searching
#[derive(Clone)]
pub enum BookmarkFilter {
    Id(String),
    UserId(String),
}

/// Lists bookmark with filters and converts into query
pub struct ListBookmark<'a> {
    repo: &'a BookmarkRepo,
    filters: Vec<BookmarkFilter>,
}

impl ListBookmark<'_> {
    /// Adds filter into the query
    pub fn filter(self, filter: BookmarkFilter) -> Self {
        let Self { repo, mut filters } = self;
        filters.push(filter);
        Self { repo, filters }
    }

    /// Runs the query of bookmark listing
    pub async fn query(self, db: &mut impl Ops) -> Result<Vec<Bookmark>, Error> {
        let Self { repo, filters } = self;

        let len = filters.len();
        let (stmts, args) = filters
            .into_iter()
            .map(|f| match f {
                BookmarkFilter::Id(id) => ("id = ?", vec![id.into()]),
                BookmarkFilter::UserId(user_id) => ("user_id = ?", vec![user_id.into()]),
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
            .map(Bookmark::try_from)
            .collect::<Result<_, _>>()
            .map_err(Error::Database)
    }
}
