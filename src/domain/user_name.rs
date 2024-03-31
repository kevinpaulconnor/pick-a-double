use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests {
    use crate::domain::UserName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn first_last_name_with_100_grapheme_are_valid() {
        let first = "😜".repeat(100);
        let last = "😜".repeat(100);
        assert_ok!(UserName::parse(first, last));
    }

    #[test]
    fn first_last_name_longer_than_100_grapheme_are_invalid() {
        let first = "😜".repeat(101);
        let last = "😜".repeat(101);
        assert_err!(UserName::parse(first, last));
    }

    #[test]
    fn whitespace_only_names_are_invalid() {
        let first = " ".to_string();
        let last = " ".to_string();
        assert_err!(UserName::parse(first, last));
    }

    #[test]
    fn empty_names_are_invalid() {
        let first = "".to_string();
        let last = "".to_string();
        assert_err!(UserName::parse(first, last));
    }

    #[test]
    fn names_with_forbidden_characters_are_invalid() {
        for name in &["/", "(", ")", "\"", "<", ">", "\\", "{", "}"] {
            let first = "first".to_string();
            let last = format!("last{}", name);
            assert_err!(UserName::parse(first, last));
        }
        for name in &["/", "(", ")", "\"", "<", ">", "\\", "{", "}"] {
            let first = format!("last{}", name);
            let last = "last".to_string();
            assert_err!(UserName::parse(first, last));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_correctly() {
        let first = "first".to_string();
        let last = "last".to_string();
        assert_ok!(UserName::parse(first, last));
    }
}

#[derive(Debug)]
pub struct UserName {
    pub first: UserNamePart,
    pub last: UserNamePart,
}

#[derive(Debug)]
pub struct UserNamePart(String);

impl AsRef<str> for UserNamePart {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn is_valid_name(s: &String) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();
    let is_too_long = s.graphemes(true).count() > 100;
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));
    if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
        return false;
    }
    true
}

impl UserName {
    pub fn parse(first: String, last: String) -> Result<UserName, String> {
        if !is_valid_name(&first) {
            return Err(format!("{} is not a valid first name", first));
        }
        if !is_valid_name(&last) {
            return Err(format!("{} is not a valid last name", last));
        }
        Ok(UserName {
            first: UserNamePart(first),
            last: UserNamePart(last),
        })
    }
}
