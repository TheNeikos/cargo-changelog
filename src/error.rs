use std::path::PathBuf;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error("IO")]
    Io(#[from] std::io::Error),

    #[error("UTF8 error")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("git error")]
    Git(#[from] git2::Error),

    #[error("TOML deserialization error")]
    Toml(#[from] toml::de::Error),

    #[error("Time formatting error")]
    TimeFormat(#[from] time::error::Format),

    #[error("Error getting cargo meta information")]
    Cargo(#[from] cargo_metadata::Error),

    #[error("Error in handlebars template")]
    HandlebarsTemplate(#[from] handlebars::TemplateError),

    #[error("Error during template rendering")]
    HandlebarsRender(#[from] handlebars::RenderError),

    #[error("Error while walking directory")]
    WalkDir(#[from] walkdir::Error),

    #[error("Repository has no worktree")]
    NoWorkTree,

    #[error("Configuration file does not exist: {0}")]
    ConfigDoesNotExist(PathBuf),

    #[error("Not a file: {0}")]
    NotAFile(PathBuf),

    #[error("No version found in Cargo.toml, that should never happen...")]
    NoVersionInCargoToml,

    #[error(
        "Versions are not all the same in the workspace, cannot decide what you want to release!"
    )]
    WorkspaceVersionsNotEqual,

    #[error("EDITOR and VISUAL are not set, cannot find editor")]
    EditorEnvNotSet,

    #[error("Environment variable '{0}' is not unicode")]
    EnvNotUnicode(String),

    #[error("Fragment Error: {}", .1.display())]
    FragmentError(#[source] FragmentError, PathBuf),

    #[error("Version error")]
    Version(#[from] VersionError),

    #[error("Text provider error")]
    TextProvider(#[from] TextProviderError),

    #[error("Verification failed")]
    Verification(#[related] Vec<VerificationError>),
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum FragmentError {
    #[error("IO")]
    Io(#[from] std::io::Error),

    #[error("Expected header seperator: '---' or '+++', found: '{0}'")]
    ExpectedSeperator(String),

    #[error("Header seperator '---' or '+++' missing")]
    HeaderSeperatorMissing,

    #[error("TOML serialization error")]
    TomlSer(#[from] toml::ser::Error),

    #[error("TOML deserialization error")]
    TomlDe(#[from] toml::de::Error),

    #[error("YAML error")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Type Error: Expected {exp}, got {recv}")]
    DataType { exp: String, recv: String },

    #[error("Error during interactive session")]
    Interactive(#[from] InteractiveError),
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum VersionError {
    #[error("UTF8 Error with path: {}", .0.display())]
    Utf8(PathBuf),
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TextProviderError {
    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[error("UTF8 Error")]
    Utf8(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum VerificationError {
    #[error("Version error")]
    Version(#[from] VersionError),

    #[error("Error while parsing fragment {0}")]
    FragmentParsing(PathBuf, #[source] FragmentError),

    #[error("Error while walking directory")]
    WalkDir(#[from] walkdir::Error),
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum InteractiveError {
    #[error("User interrupted interactive session")]
    Interrupted,

    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[error("Type Error: Expected {}, got {}", .0.type_name(), .1.type_name())]
    TypeError(
        crate::fragment::FragmentDataType,
        crate::fragment::FragmentData,
    ),

    #[error("Required value '{}' not provided", .0)]
    RequiredValueNotProvided(String),

    #[error("Failed to parse intefer")]
    ParseInt(#[from] std::num::ParseIntError),
}
