use unicode_segmentation::UnicodeSegmentation;

pub struct UserName {
    pub first: UserNamePart,
    pub last: UserNamePart,
}

pub struct UserNamePart(String);

impl AsRef<str> for UserNamePart {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
pub struct NewUser {
    pub name: UserName,
    pub email: String,
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
    pub fn parse(first: String, last: String) -> UserName {
        if !is_valid_name(&first) {
            panic!("Invalid first name");
        }
        if !is_valid_name(&last) {
            panic!("Invalid last name");
        }
        UserName {
            first: UserNamePart(first),
            last: UserNamePart(last),
        }
    }
}
