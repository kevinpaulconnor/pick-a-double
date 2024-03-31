use crate::domain::user_email::UserEmail;
use crate::domain::user_name::UserName;

pub struct NewUser {
    pub name: UserName,
    pub email: UserEmail,
}
