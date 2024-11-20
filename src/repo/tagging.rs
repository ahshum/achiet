use super::{util, Error};
use crate::{
    database::{Ops, Row, Value},
    model::{Tagging, TaggingType},
};

/// Manages database operations of tagging
pub struct TaggingRepo {
    table_name: String,
}

impl TaggingRepo {
    pub fn new(tagging_type: TaggingType) -> Self {
        Self {
            table_name: format!("{}_tagging", tagging_type),
        }
    }

    /// Lists taggings with filters
    pub fn list(&self) -> ListTagging<'_> {
        ListTagging {
            repo: self,
            filters: Vec::new(),
        }
    }

    /// Creates tagging with data
    pub async fn create(&self, db: &mut impl Ops, data: Tagging) -> Result<Tagging, Error> {
        let tagging = Tagging {
            id: util::new_id(),
            ..data
        };

        let row = Row::try_from(tagging.clone()).map_err(Error::Database)?;
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
            .and_then(util::expect_single_row(tagging))
    }

    /// Updates tagging with data
    pub async fn update(&self, db: &mut impl Ops, data: Tagging) -> Result<Tagging, Error> {
        let tagging = Tagging { ..data };

        let fields = ["item_id", "tag_id", "value", "order_index"];
        let (stmts, args) = Row::try_from(tagging.clone())
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

        let mut sql = db.sql();
        sql.push("UPDATE ")
            .push(&self.table_name)
            .push(" SET ")
            .push(stmts.join(", "))
            .bind_many(args)
            .push(" WHERE id = ?")
            .bind(tagging.id.clone());

        db.execute(sql)
            .await
            .map_err(Error::Database)
            .and_then(util::expect_single_row(tagging))
    }

    /// Delete tagging based on the `id` in data
    pub async fn delete(&self, db: &mut impl Ops, data: Tagging) -> Result<(), Error> {
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

/// Tagging filter for searching
#[derive(Clone)]
pub enum TaggingFilter {
    TagId(String),
    ItemIdIn(Vec<String>),
}

/// Lists tagging with filters and converts into query
pub struct ListTagging<'a> {
    repo: &'a TaggingRepo,
    filters: Vec<TaggingFilter>,
}

impl ListTagging<'_> {
    /// Adds filter into the query
    pub fn filter(self, filter: TaggingFilter) -> Self {
        let Self { repo, mut filters } = self;
        filters.push(filter);
        Self { repo, filters }
    }

    /// Runs the query of tagging listing
    pub async fn query(self, db: &mut impl Ops) -> Result<Vec<Tagging>, Error> {
        let Self { repo, filters } = self;

        let len = filters.len();
        let (stmts, args) = filters
            .clone()
            .into_iter()
            .map(|f| match f {
                TaggingFilter::TagId(id) => ("tag_id = ?".to_string(), vec![id.into()]),
                TaggingFilter::ItemIdIn(list) => (
                    format!("item_id IN ({})", vec!["?"; list.len()].join(",")),
                    list.into_iter().map(|id| id.into()).collect::<Vec<_>>(),
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
        sql.push(" ORDER BY order_index");

        db.query(sql)
            .await
            .map_err(Error::Database)?
            .into_iter()
            .map(Tagging::try_from)
            .collect::<Result<_, _>>()
            .map_err(Error::Database)
    }
}
