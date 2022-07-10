use unicode_segmentation::UnicodeSegmentation;

pub struct NewMarking {
    pub name: MarkingName,
    pub definition_type: MarkingDefinitionType,
    pub definition: MarkingDefinition,
}

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

#[derive(Debug)]
pub struct MarkingDefinitionType(MarkingType);

impl MarkingDefinitionType {
    pub fn parse(s: String) -> Result<MarkingDefinitionType, String> {
        Ok(Self(s.try_into()?))
    }
}

impl AsRef<str> for MarkingDefinitionType {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub enum MarkingType {
    Tlp,
    Statement,
}

impl MarkingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MarkingType::Tlp => "tlp",
            MarkingType::Statement => "statement",
        }
    }
}

impl TryFrom<String> for MarkingType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "tlp" => Ok(Self::Tlp),
            "statement" => Ok(Self::Statement),
            other => Err(format!(
                "{} is not a supported marking type. Use either 'tlp' or 'statement'.",
                other
            )),
        }
    }
}

#[cfg(test)]
mod name_tests {
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

#[cfg(test)]
mod definition_tests {
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

#[cfg(test)]
mod definition_type_tests {
    use crate::domain::MarkingDefinitionType;
    use claim::{assert_err, assert_ok};

    #[test]
    fn tlp_type_is_valid() {
        let definition_type = "tlp".to_string();
        assert_ok!(MarkingDefinitionType::parse(definition_type));
    }

    #[test]
    fn statement_type_is_valid() {
        let definition_type = "statement".to_string();
        assert_ok!(MarkingDefinitionType::parse(definition_type));
    }

    #[test]
    fn all_other_types_are_rejected() {
        let definition_type = "something random +)(*".to_string();
        assert_err!(MarkingDefinitionType::parse(definition_type));
    }
}
