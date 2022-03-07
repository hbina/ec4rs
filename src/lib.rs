#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![deny(missing_docs)]

mod ecfile;
mod ecparser;
mod error;
mod fallback;
mod glob;
mod linereader;
mod properties;
pub mod property;
mod section;
pub mod version;
#[cfg(test)]
mod tests;

pub use ecfile::{EcFile, EcFiles};
pub use ecparser::EcParser;
pub use error::{Error, ParseError};
pub use properties::{Properties, PropertiesSource, RawValue};
pub use section::Section;

/// Retrieve the [Properties] for a file at the given path.
///
/// This is the simplest way to use this library in an EditorConfig integration or plugin.
///
/// This function does not canonicalize the path,
/// but will join relative paths onto the current working directory.
///
/// EditorConfig files are assumed to be named `.editorconfig`.
/// If not, use [get_config_at_path_for]
pub fn config_for(
	path: impl AsRef<std::path::Path>
) -> Result<Properties, Error> {
	config_at_path_for(path, None as Option<&std::path::Path>)
}

/// Retrieve the [Properties] for a file at the given path.
///
/// This function does not canonicalize the path,
/// but will join relative paths onto the current working directory.
///
/// If the provided config path is absolute, uses the EditorConfig file at that path. If it's relative, joins it onto every ancestor of the target file, and looks for config files at those paths.
/// If it's `None`, EditorConfig files are assumed to be named `.editorconfig`.
pub fn config_at_path_for(
	target_path: impl AsRef<std::path::Path>,
	config_path_override: Option<impl AsRef<std::path::Path>>
) -> Result<Properties, Error> {
	let mut retval = Properties::new();
	EcFiles::open(&target_path, config_path_override)?.apply_to(&mut retval, &target_path)?;
	retval.use_fallbacks();
	Ok(retval)
}
