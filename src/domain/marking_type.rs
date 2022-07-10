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
