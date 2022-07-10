use crate::domain::{MarkingDefinition, MarkingDefinitionType, MarkingName};

pub struct NewMarking {
    pub name: MarkingName,
    pub definition_type: MarkingDefinitionType,
    pub definition: MarkingDefinition,
}
