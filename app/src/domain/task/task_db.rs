#[cfg(feature = "ssr")]
pub mod db {

    use anyhow::anyhow;
    use sea_orm::ActiveValue::Set;
    use time::OffsetDateTime;

    use sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, TryIntoModel,
    };

    use crate::common::DbPool;

    use crate::common::api_error::ApiError;
    use crate::database::tasks::{self, Entity as Tasks};
    use crate::domain::task::model::task::Task;

    pub async fn get_tasks_from_db(
        pool: &DbPool,
        user_id: Option<i32>,
    ) -> Result<Vec<tasks::Model>, sea_orm::DbErr> {
        Tasks::find()
            .filter(tasks::Column::UserId.eq(user_id))
            .filter(tasks::Column::DeletedAt.is_null())
            .all(pool)
            .await
    }

    pub async fn get_task_from_db(
        pool: &DbPool,
        id: i32,
        user_id: Option<i32>,
    ) -> Result<Option<tasks::Model>, sea_orm::DbErr> {
        Tasks::find()
            .filter(tasks::Column::Id.eq(id))
            .filter(tasks::Column::UserId.eq(user_id))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(pool)
            .await
    }

    pub async fn get_task_by_title_from_db(
        pool: &DbPool,
        title: &Option<String>,
        user_id: i32,
    ) -> Result<Option<tasks::Model>, sea_orm::DbErr> {
        Tasks::find()
            .filter(tasks::Column::Title.eq(title.to_owned()))
            .filter(tasks::Column::UserId.eq(user_id))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(pool)
            .await
    }

    pub async fn delete_task_in_db(
        pool: &DbPool,
        id: i32,
        user_id: Option<i32>,
    ) -> Result<i32, sea_orm::DbErr> {
        let task = Tasks::find()
            .filter(tasks::Column::Id.eq(id))
            .filter(tasks::Column::UserId.eq(user_id))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(pool)
            .await?
            .unwrap();

        let mut active_task = task.into_active_model();

        let now = time::OffsetDateTime::now_utc();
        let rfc2822_string = time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc2822)
            .unwrap_or_else(|_| panic!("failed format now={}", now));
        active_task.deleted_at = Set(Some(rfc2822_string));

        let saved_task = active_task.save(pool).await?;

        Ok(saved_task.id.unwrap())
    }

    pub async fn update_task_in_db(
        pool: &DbPool,
        patch: &Task,
        user_id: Option<i32>,
    ) -> anyhow::Result<tasks::Model> {
        let task = Tasks::find()
            .filter(tasks::Column::Id.eq(patch.id))
            .filter(tasks::Column::UserId.eq(user_id))
            .filter(tasks::Column::DeletedAt.is_null())
            .one(pool)
            .await?
            .unwrap();

        let old_completed_at = task.completed_at.to_owned();

        let mut active_task = task.into_active_model();

        if let Some(title) = &patch.title {
            active_task.title.set_if_not_equals(title.to_owned());
        }
        if let Some(description) = &patch.description {
            active_task.description.set_if_not_equals(Some(description.to_owned()));
        }
        if let Some(priority) = &patch.priority {
            active_task.priority.set_if_not_equals(Some(priority.to_owned()));
        }

        let new_completed_at = match &patch.completed_at {
            Some(completed_at) => Some(parse_to_datetime_utc(completed_at)?),
            None => None,
        };

        if (new_completed_at.is_some() && old_completed_at.is_none()) || new_completed_at.is_none()
        {
            active_task.completed_at.set_if_not_equals(new_completed_at);
        }

        if !active_task.is_changed() {
            return Err(anyhow!(ApiError::Db("Нечего менять!".to_owned())));
        }

        let active_task = active_task.update(pool).await?;

        active_task.try_into_model().map_err(|err| anyhow!(err))
    }

    pub async fn create_task_in_db(
        pool: &DbPool,
        task: &Task,
        user_id: i32,
    ) -> anyhow::Result<tasks::Model> {
        let completed_at = match &task.completed_at {
            Some(completed_at) => Some(parse_to_datetime_utc(completed_at)?),
            None => None,
        };

        let saved_task = tasks::ActiveModel {
            priority: Set(task.priority.to_owned()),
            title: Set(task.title.to_owned().unwrap()),
            description: Set(task.description.to_owned()),
            user_id: Set(Some(user_id)),
            is_default: Set(Some(false)),
            completed_at: Set(completed_at),
            ..Default::default()
        }
        .save(pool)
        .await?;

        saved_task.try_into_model().map_err(|err| anyhow!(err))
    }

    fn parse_to_datetime_utc(rfc2822_str: &str) -> Result<OffsetDateTime, time::error::Parse> {
        <time::OffsetDateTime>::parse(
            rfc2822_str,
            &time::format_description::well_known::Rfc2822,
        )
    }
}
