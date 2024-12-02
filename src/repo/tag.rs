use super::{util, Error};
use crate::{
    database::{Ops, Row, Value},
    model::Tag,
};

/// Manages database operations of tag
pub struct TagRepo {
    table_name: String,
}

impl TagRepo {
    pub fn new() -> Self {
        Self {
            table_name: "tag".to_string(),
        }
    }

    /// Lists tags with filters
    pub fn list(&self) -> ListTag<'_> {
        ListTag {
            repo: self,
            filters: Vec::new(),
        }
    }

    /// Creates tag with data
    pub async fn create(&self, db: &mut impl Ops, data: Tag) -> Result<Tag, Error> {
        let now = util::now();
        let tag = Tag {
            id: util::new_id(),
            created_at: now.clone(),
            updated_at: now,
            ..data
        };

        let row = Row::try_from(tag.clone()).map_err(Error::Database)?;
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
            .and_then(util::expect_single_row(tag))
    }

    /// Updates tag with data
    pub async fn update(&self, db: &mut impl Ops, data: Tag) -> Result<Tag, Error> {
        let now = util::now();
        let tag = Tag {
            updated_at: now,
            ..data
        };

        let fields = [
            "path",
            "prefix",
            "name",
            "label",
            "parent_id",
            "depth",
            "value_type",
            "updated_at",
        ];
        let (stmts, args) = Row::try_from(tag.clone())
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
            .bind(tag.id.clone());

        db.execute(sql)
            .await
            .map_err(Error::Database)
            .and_then(util::expect_single_row(tag))
    }

    /// Delete tag based on the `id` in data
    pub async fn delete(&self, db: &mut impl Ops, data: Tag) -> Result<(), Error> {
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

/// Tag filter for searching
#[derive(Clone)]
pub enum TagFilter {
    Id(String),
    IdIn(Vec<String>),
    UserId(String),
    NameLike(String),
    PathIn(Vec<String>),
}

/// Lists tag with filters and converts into query
pub struct ListTag<'a> {
    repo: &'a TagRepo,
    filters: Vec<TagFilter>,
}

impl ListTag<'_> {
    /// Adds filter into the query
    pub fn filter(self, filter: TagFilter) -> Self {
        let Self { repo, mut filters } = self;
        filters.push(filter);
        Self { repo, filters }
    }

    /// Runs the query of tag listing
    pub async fn query(self, db: &mut impl Ops) -> Result<Vec<Tag>, Error> {
        let Self { repo, filters } = self;

        let len = filters.len();
        let (stmts, args) = filters
            .clone()
            .into_iter()
            .map(|f| match f {
                TagFilter::Id(id) => ("id = ?".to_string(), vec![id.into()]),
                TagFilter::IdIn(ids) => (
                    format!("id IN ({})", vec!["?"; ids.len()].join(",")),
                    ids.into_iter().map(|v| v.into()).collect::<Vec<_>>(),
                ),
                TagFilter::UserId(user_id) => ("user_id = ?".to_string(), vec![user_id.into()]),
                TagFilter::NameLike(name) => (
                    "name LIKE ?".to_string(),
                    vec![("%".to_string() + &name + "%").into()],
                ),
                TagFilter::PathIn(paths) => (
                    format!("path IN ({})", vec!["?"; paths.len()].join(",")),
                    paths.into_iter().map(|p| p.into()).collect::<Vec<_>>(),
                ),
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
            .map(Tag::try_from)
            .collect::<Result<_, _>>()
            .map_err(Error::Database)
    }
}
