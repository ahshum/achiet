use crate::{api, app::AppState, model, repo};

pub enum Task {
    Empty,
    TagUpdated(model::Tag),
}

impl Task {
    pub async fn run(self, app_state: &AppState) -> Result<(), ()> {
        match self {
            Task::Empty => {
                log::debug!("discarded");
                Ok(())
            }

            Task::TagUpdated(tag) => {
                log::info!("Task::TagUpdated start - {}", tag.path);
                if tag.depth == 1 || tag.parent_id.is_some() {
                    return Ok(());
                }

                let mut splits = tag.path.split("/").collect::<Vec<_>>();
                let _ = splits.pop();
                let parent_path = splits.join("/");
                log::debug!(
                    "Task::TagUpdated pre process - {} - {:?} - {:?}",
                    tag.path,
                    splits,
                    parent_path
                );

                let parent = repo::tag::sync_tags(
                    app_state,
                    tag.user_id.clone(),
                    vec![model::Tag::from_path(parent_path)],
                )
                .await
                .map_err(|_| ())?
                .remove(0);

                log::debug!("Task::TagUpdated parent of {:?} is {:?}", tag, parent);
                let _ = repo::tag::update_tags(
                    app_state,
                    vec![model::Tag {
                        parent_id: Some(parent.id),
                        ..tag
                    }],
                )
                .await
                .map_err(|_| ())?;

                Ok(())
            }
        }
    }
}
