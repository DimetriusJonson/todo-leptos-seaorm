use crate::domain::user::model::user::User;

#[derive(Clone, Debug)]
pub struct SecurityContext {
    pub user: Option<User>,
}
