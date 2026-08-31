use uuid::Uuid;

pub struct AuthUser {
    user_id: Uuid,
}

impl AuthUser {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
}
