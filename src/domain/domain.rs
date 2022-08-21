use unicode_segmentation::UnicodeSegmentation;
pub struct SubscriberName(String);

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        let is_empty_white_space = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_fobidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if (is_empty_white_space || is_too_long || contains_fobidden_characters) {
            panic!("{} is not a valid subscriber name", s);
        } else {
            Self(s)
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
