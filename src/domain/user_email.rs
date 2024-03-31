use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Debug, Deserialize)]
pub struct UserEmail {
    #[validate(email)]
    as_str: String,
}

impl UserEmail {
    pub fn parse(s: String) -> Result<UserEmail, String> {
        let email = UserEmail { as_str: s };
        match UserEmail::validate(&email) {
            Ok(_) => Ok(email),
            Err(_e) => Err(format!("{} is not a valid user email", email.as_str)),
        }
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.as_str
    }
}

#[cfg(test)]
mod tests {
    use super::UserEmail;
    use claims::assert_err;

    #[test]
    fn empty_string_is_rejected() {
        assert_err!(UserEmail::parse("".to_string()));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        assert_err!(UserEmail::parse("email.com".to_string()));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        assert_err!(UserEmail::parse("@example.com".to_string()));
    }
}
