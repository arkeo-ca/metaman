use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct MarkingName(String);

impl MarkingName {
    pub fn parse(s: String) -> Result<MarkingName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let mut alphabet = (10..36)
            .map(|i| char::from_digit(i, 36).unwrap())
            .collect::<Vec<_>>();
        alphabet.push('_');
        let only_allowed_characters = s.chars().all(|g| alphabet.contains(&g));

        if is_empty_or_whitespace || is_too_long || !only_allowed_characters {
            Err(format!("{} is not a valid name for a marking.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for MarkingName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::MarkingName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "e".repeat(256);
        assert_ok!(MarkingName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(MarkingName::parse(name));
    }

    #[test]
    fn a_name_with_only_whitespace_is_rejected() {
        let name = "".to_string();
        assert_err!(MarkingName::parse(name));
    }

    #[test]
    fn an_empty_name_is_rejected() {
        let name = "".to_string();
        assert_err!(MarkingName::parse(name));
    }

    #[test]
    fn name_with_invalid_character_are_rejected() {
        for name in &[
            "foo1",
            "foo bar",
            "foo's bar",
            "<foo>",
            "Foo",
            "FOO",
            "foo-bar",
        ] {
            let name = name.to_string();
            assert_err!(MarkingName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "this_would_be_a_valid_name".to_string();
        assert_ok!(MarkingName::parse(name));
    }
}
