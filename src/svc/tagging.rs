use super::{Error, TagSvc};
use crate::{
    database::Ops,
    field,
    model::{Tag, Tagging, TaggingType},
    repo::{TaggingFilter, TaggingRepo},
};
use std::collections::HashMap;

pub struct TaggingSvc {
    tag_svc: TagSvc,
    tagging: TaggingRepo,
}

impl TaggingSvc {
    pub fn new(tagging_type: TaggingType) -> Self {
        Self {
            tag_svc: TagSvc::new(),
            tagging: TaggingRepo::new(tagging_type),
        }
    }

    pub async fn list_taggings_by_item_ids(
        &self,
        db: &mut impl Ops,
        item_ids: Vec<field::Id>,
    ) -> Result<HashMap<String, Vec<(Tag, Tagging)>>, Error> {
        let tgs = self
            .tagging
            .list()
            .filter(TaggingFilter::ItemIdIn(item_ids))
            .query(db)
            .await
            .map_err(Error::Repo)?;

        if tgs.len() == 0 {
            return Ok(HashMap::new());
        }

        let ts = self
            .tag_svc
            .list_by_ids(db, tgs.clone().into_iter().map(|tg| tg.tag_id).collect())
            .await?;

        Ok(tgs.into_iter().fold(
            HashMap::<String, Vec<(Tag, Tagging)>>::new(),
            |mut items_map, tg| {
                let t = ts.iter().find(|t| t.id == tg.tag_id).unwrap();
                items_map
                    .entry(tg.item_id.clone())
                    .or_insert_with(Vec::new)
                    .push((t.clone(), tg));
                items_map
            },
        ))
    }

    pub fn split_tag_str(tag_str: String) -> (String, Option<String>) {
        let (path, value) = match tag_str.split_once(':') {
            Some((path, value)) => (path.to_string(), Some(value.to_string())),
            None => (tag_str, None),
        };
        (Tag::from_path(path).path, value)
    }

    pub async fn find_or_create_tags(
        &self,
        db: &mut impl Ops,
        user_id: field::Id,
        tag_strs: Vec<String>,
    ) -> Result<Vec<Tag>, Error> {
        self.tag_svc
            .find_or_create_tags(
                db,
                user_id,
                tag_strs
                    .into_iter()
                    .map(|s| Self::split_tag_str(s).0)
                    .collect::<Vec<_>>(),
            )
            .await
    }

    pub async fn replace_tags(
        &self,
        db: &mut impl Ops,
        user_id: field::Id,
        item_id: field::Id,
        tag_strs: Vec<String>,
    ) -> Result<Vec<(Tag, Tagging)>, Error> {
        let mut old_tgs = self
            .list_taggings_by_item_ids(db, vec![item_id.clone()])
            .await?
            .remove(&item_id)
            .unwrap_or_default();

        let tags = self
            .find_or_create_tags(db, user_id, tag_strs.clone())
            .await?;

        let tg_pairs = tag_strs
            .into_iter()
            .map(Self::split_tag_str)
            .collect::<Vec<_>>();

        let mut ret = Vec::new();

        // upsert
        for (i, (path, value)) in tg_pairs.into_iter().enumerate() {
            let tag = tags.iter().find(|t| t.path == path).unwrap();
            match old_tgs.iter().position(|(_, tg)| tg.tag_id == tag.id) {
                Some(idx) => {
                    let (_, old_tg) = old_tgs.remove(idx);
                    let tg = self
                        .tagging
                        .update(
                            db,
                            Tagging {
                                value: value.clone(),
                                order_index: i.clone(),
                                ..old_tg
                            },
                        )
                        .await
                        .map_err(Error::Repo)?;
                    ret.push((tag.clone(), tg));
                }
                None => {
                    let tg = self
                        .tagging
                        .create(
                            db,
                            Tagging {
                                tag_id: tag.id.clone(),
                                item_id: item_id.clone(),
                                value: value.clone(),
                                order_index: i.clone(),
                                ..Tagging::default()
                            },
                        )
                        .await
                        .map_err(Error::Repo)?;
                    ret.push((tag.clone(), tg));
                }
            }
        }

        // delete
        for (_, tg) in old_tgs {
            self.tagging.delete(db, tg).await.map_err(Error::Repo)?;
        }

        Ok(ret)
    }
}
