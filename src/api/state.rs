use crate::model::User;

#[derive(Clone, Debug)]
pub enum AuthenticationState {
    Guest,
    User(User),
}

impl AuthenticationState {
    pub fn user(&self) -> Option<User> {
        match self {
            AuthenticationState::User(user) => Some(user.clone()),
            _ => None,
        }
    }

    pub fn user_id(&self) -> Option<String> {
        self.user().map(|u| u.id)
    }
}
