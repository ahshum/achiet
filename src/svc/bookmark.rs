use super::{Error, TaggingSvc};
use crate::{
    database::Ops,
    field,
    model::{Bookmark, Tag, Tagging, TaggingType},
    repo::{BookmarkFilter, BookmarkRepo},
};

pub struct BookmarkSvc {
    bookmark: BookmarkRepo,
    tagging_svc: TaggingSvc,
}

impl BookmarkSvc {
    pub fn new() -> Self {
        Self {
            bookmark: BookmarkRepo::new(),
            tagging_svc: TaggingSvc::new(TaggingType::Bookmark),
        }
    }

    async fn list_by(
        &self,
        db: &mut impl Ops,
        filters: Vec<BookmarkFilter>,
    ) -> Result<Vec<(Bookmark, Vec<(Tag, Tagging)>)>, Error> {
        let bookmarks = filters
            .into_iter()
            .fold(self.bookmark.list(), |query, f| query.filter(f))
            .query(db)
            .await
            .map_err(Error::Repo)?;

        let mut bm_tg_map = self
            .tagging_svc
            .list_taggings_by_item_ids(db, bookmarks.clone().into_iter().map(|b| b.id).collect())
            .await?;

        Ok(bookmarks
            .into_iter()
            .map(|b| {
                let tg = bm_tg_map.remove(&b.id).unwrap_or_default();
                (b, tg)
            })
            .collect::<Vec<_>>())
    }

    pub async fn list_by_user(
        &self,
        db: &mut impl Ops,
        user_id: field::Id,
    ) -> Result<Vec<(Bookmark, Vec<(Tag, Tagging)>)>, Error> {
        self.list_by(db, vec![BookmarkFilter::UserId(user_id)])
            .await
    }

    pub async fn find_by_user_and_id(
        &self,
        db: &mut impl Ops,
        user_id: field::Id,
        bm_id: field::Id,
    ) -> Result<(Bookmark, Vec<(Tag, Tagging)>), Error> {
        self.list_by(
            db,
            vec![BookmarkFilter::UserId(user_id), BookmarkFilter::Id(bm_id)],
        )
        .await?
        .into_iter()
        .next()
        .ok_or(Error::NotFound)
    }

    pub async fn create(&self, db: &mut impl Ops, data: Bookmark) -> Result<Bookmark, Error> {
        self.bookmark.create(db, data).await.map_err(Error::Repo)
    }

    pub async fn update(&self, db: &mut impl Ops, data: Bookmark) -> Result<Bookmark, Error> {
        self.bookmark.update(db, data).await.map_err(Error::Repo)
    }

    pub async fn delete(&self, db: &mut impl Ops, data: Bookmark) -> Result<(), Error> {
        self.bookmark.delete(db, data).await.map_err(Error::Repo)
    }

    pub async fn replace_tags(
        &self,
        db: &mut impl Ops,
        bookmark: Bookmark,
        tag_strs: Vec<String>,
    ) -> Result<Vec<(Tag, Tagging)>, Error> {
        self.tagging_svc
            .replace_tags(db, bookmark.user_id.clone(), bookmark.id.clone(), tag_strs)
            .await
    }
}
