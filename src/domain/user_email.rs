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
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

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

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        UserEmail::parse(valid_email.0).is_ok()
    }
}
