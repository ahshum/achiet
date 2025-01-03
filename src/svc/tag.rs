use super::Error;
use crate::{
    database::Ops,
    field,
    model::Tag,
    repo::{TagFilter, TagRepo},
};

/// Tag service
pub struct TagSvc {
    tag: TagRepo,
}

impl TagSvc {
    pub fn new() -> Self {
        Self {
            tag: TagRepo::new(),
        }
    }

    async fn list_by(&self, db: &mut impl Ops, filters: Vec<TagFilter>) -> Result<Vec<Tag>, Error> {
        filters
            .into_iter()
            .fold(self.tag.list(), |query, f| query.filter(f))
            .query(db)
            .await
            .map_err(Error::Repo)
    }

    pub async fn list_by_user(
        &self,
        db: &mut impl Ops,
        user_id: field::Id,
    ) -> Result<Vec<Tag>, Error> {
        self.list_by(db, vec![TagFilter::UserId(user_id)]).await
    }

    pub async fn find_by_user_and_id(
        &self,
        db: &mut impl Ops,
        user_id: field::Id,
        tag_id: field::Id,
    ) -> Result<Tag, Error> {
        self.list_by(db, vec![TagFilter::UserId(user_id), TagFilter::Id(tag_id)])
            .await?
            .into_iter()
            .next()
            .ok_or(Error::NotFound)
    }

    pub async fn list_by_ids(
        &self,
        db: &mut impl Ops,
        ids: Vec<field::Id>,
    ) -> Result<Vec<Tag>, Error> {
        self.list_by(db, vec![TagFilter::IdIn(ids)]).await
    }

    pub async fn find_or_create_tags(
        &self,
        db: &mut impl Ops,
        user_id: field::Id,
        tag_strs: Vec<String>,
    ) -> Result<Vec<Tag>, Error> {
        let old_tags = self
            .list_by(
                db,
                vec![
                    TagFilter::UserId(user_id.clone()),
                    TagFilter::PathIn(tag_strs.clone()),
                ],
            )
            .await?;

        let mut ret = old_tags.clone();

        let mut new_tags = tag_strs
            .into_iter()
            .filter(|s| old_tags.iter().find(|t| t.path == *s).is_none())
            .map(|s| Tag {
                user_id: user_id.clone(),
                ..Tag::from_path(s)
            })
            .collect::<Vec<_>>();

        // create missing parent
        Box::pin(async {
            let parent_strs = new_tags
                .clone()
                .into_iter()
                .filter(|t| t.parent_id.is_none() && t.prefix != "/")
                .map(|t| t.prefix)
                .collect::<Vec<_>>();
            if parent_strs.len() > 0 {
                self.find_or_create_tags(db, user_id.clone(), parent_strs)
                    .await?
                    .into_iter()
                    .for_each(|pt| {
                        new_tags
                            .iter_mut()
                            .filter(|t| t.prefix == pt.path)
                            .for_each(|t| t.parent_id = Some(pt.id.clone()));
                    });
            }
            Ok(())
        })
        .await?;

        for data in new_tags {
            ret.push(self.create(db, data).await?);
        }

        Ok(ret)
    }

    pub async fn create(&self, db: &mut impl Ops, data: Tag) -> Result<Tag, Error> {
        self.tag.create(db, data).await.map_err(Error::Repo)
    }

    pub async fn update(&self, db: &mut impl Ops, data: Tag) -> Result<Tag, Error> {
        self.tag.update(db, data).await.map_err(Error::Repo)
    }

    pub async fn delete(&self, db: &mut impl Ops, data: Tag) -> Result<(), Error> {
        self.tag.delete(db, data).await.map_err(Error::Repo)
    }
}
