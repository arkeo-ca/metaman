use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct MarkingDefinition(String);

impl MarkingDefinition {
    pub fn parse(s: String) -> Result<MarkingDefinition, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid name for a marking.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for MarkingDefinition {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::MarkingDefinition;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_definition_is_valid() {
        let definition = "é".repeat(256);
        assert_ok!(MarkingDefinition::parse(definition));
    }

    #[test]
    fn a_definition_longer_than_256_graphemes_is_rejected() {
        let definition = "a".repeat(257);
        assert_err!(MarkingDefinition::parse(definition));
    }

    #[test]
    fn a_definition_with_only_whitespace_is_rejected() {
        let definition = "      ".to_string();
        assert_err!(MarkingDefinition::parse(definition));
    }

    #[test]
    fn an_empty_definition_is_rejected() {
        let definition = "".to_string();
        assert_err!(MarkingDefinition::parse(definition));
    }

    #[test]
    fn definition_with_invalid_character_are_rejected() {
        for definition in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let definition = definition.to_string();
            assert_err!(MarkingDefinition::parse(definition));
        }
    }

    #[test]
    fn a_valid_definition_is_parsed_successfully() {
        let definition = "This could be a valid statement.".to_string();
        assert_ok!(MarkingDefinition::parse(definition));
    }
}
