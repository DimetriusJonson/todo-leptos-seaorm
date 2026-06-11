#[cfg(feature = "ssr")]
pub mod db {
    use sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
        TryIntoModel,
    };
    use crate::{
        common::DbPool,
        database::users::{self, Entity as Users},
        domain::user::model::user::User,
    };

    pub async fn get_user_from_db(
        pool: &DbPool,
        id: i64,
    ) -> Result<Option<users::Model>, sea_orm::DbErr> {
        Users::find()
            .filter(users::Column::Id.eq(id))
            .filter(users::Column::DeletedAt.is_null())
            .one(pool)
            .await
    }

    pub async fn get_user_by_name_from_db(
        pool: &DbPool,
        name: Option<String>,
    ) -> Result<Option<users::Model>, sea_orm::DbErr> {
        Users::find()
            .filter(users::Column::Username.eq(name))
            .filter(users::Column::DeletedAt.is_null())
            .one(pool)
            .await
    }

    pub async fn create_user_in_db(
        pool: &DbPool,
        user: &User,
    ) -> Result<users::Model, sea_orm::DbErr> {
        users::ActiveModel {
            username: Set(user.username.to_owned().unwrap()),
            password: Set(user.password.to_owned().unwrap()),
            ..Default::default()
        }
        .save(pool)
        .await?
        .try_into_model()
    }

    pub async fn update_user_in_db(pool: &DbPool, user: &User) -> Result<bool, sea_orm::DbErr> {
        if let Some(found_user) =
            Users::find().filter(users::Column::Id.eq(user.id)).one(pool).await?
        {
            let mut saved_user = found_user.into_active_model();
            saved_user.token = Set(user.token.to_owned());
            saved_user.save(pool).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
