use crate::error::{CompilerEditionError, ProjectTypeError};

#[derive(Default, Debug, PartialEq, Eq)]
pub enum CompilerEdition {
    E2015,
    E2018,
    E2021,
    #[default]
    E2024,
}

impl TryFrom<&str> for CompilerEdition {
    type Error = CompilerEditionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "2015" | "15" => Ok(CompilerEdition::E2015),
            "2018" | "18" => Ok(CompilerEdition::E2018),
            "2021" | "21" => Ok(CompilerEdition::E2021),
            "2024" | "24" => Ok(CompilerEdition::E2024),
            _ => Err(CompilerEditionError::InvalidEdition(value.to_string())),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ProjectType {
    #[default]
    Bin,
    Lib,
}

impl TryFrom<(bool, bool, Option<&String>)> for ProjectType {
    type Error = ProjectTypeError;
    fn try_from(value: (bool, bool, Option<&String>)) -> Result<Self, Self::Error> {
        let (lib, bin, str_type) = value;

        if lib && bin {
            return Err(ProjectTypeError::BothSpecified);
        }

        if let Some(str_type) = str_type {
            return match str_type.as_str() {
                "lib" if bin => Err(ProjectTypeError::BothSpecified),
                "bin" if lib => Err(ProjectTypeError::BothSpecified),
                "lib" => Ok(ProjectType::Lib),
                "bin" => Ok(ProjectType::Bin),
                _ => Err(ProjectTypeError::InvalidType(str_type.clone())),
            };
        }

        if lib {
            Ok(ProjectType::Lib)
        } else {
            Ok(ProjectType::Bin)
        }
    }
}
