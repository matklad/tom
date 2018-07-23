extern crate clap;
#[macro_use]
extern crate failure;
extern crate tom;

mod cargo_toml;

use std::{
    fs,
    borrow::Cow,
};
use clap::{App, Arg};
pub use cargo_toml::CargoToml;
use tom::{ast, TomlDoc};

type Result<T> = ::std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let app = App::new("cargo-ed")
        .arg(Arg::with_name("package").required(true))
        .arg(
            Arg::with_name("manifest-path")
                .takes_value(true)
                .long("manifest-path"),
        )
        .arg(
            Arg::with_name("version")
                .takes_value(true)
                .long("version")
                .required_unless("git"),
        )
        .arg(Arg::with_name("git").takes_value(true).long("git"))
        .arg(
            Arg::with_name("branch")
                .takes_value(true)
                .long("branch")
                .requires("git"),
        )
        .arg(Arg::with_name("optional").long("optional"));
    let matches = app.get_matches();

    let manifest_path = matches.value_of("manifest-path").unwrap_or("Cargo.toml");
    let dep = Dependency {
        name: matches.value_of("package").unwrap().to_string(),
        optional: matches.is_present("optional"),
        source: match matches.value_of("git") {
            Some(url) => DependencySource::Git {
                url: url.to_string(),
                version: matches.value_of("version").map(|s| s.to_string()),
                branch: matches.value_of("branch").map(|s| s.to_string()),
            },
            None => DependencySource::Version(matches.value_of("version").unwrap().to_string()),
        },
    };

    let text = fs::read_to_string(manifest_path)?;
    let mut toml = CargoToml::new(&text)?;
    toml.update_dependency(&dep)?;
    let result = toml.text();

    fs::write(manifest_path, result)?;

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub optional: bool,
    pub source: DependencySource,
}

impl Dependency {
    fn from_entry(doc: &TomlDoc, entry: ast::Entry) -> Option<Dependency> {
        let name = single_key(doc, entry)?.into_owned();
        let value = entry.value(doc);
        let mut optional = false;
        let source = match value.kind(doc) {
            ast::ValueKind::StringLit(s) => {
                DependencySource::Version(s.value(doc).into_owned())
            }
            ast::ValueKind::Dict(d) => {
                let mut url = None;
                let mut branch = None;
                let mut version = None;
                for e in d.entries(doc) {
                    match single_key(doc, e)?.as_ref() {
                        "git" => url = Some(string_value(doc, e)?),
                        "version" => version = Some(string_value(doc, e)?),
                        "branch" => branch = Some(string_value(doc, e)?),
                        "optional" => optional = bool_value(doc, e)?,
                        _ => return None,
                    }
                }
                DependencySource::Git { url: url?, version, branch }
            }
            _ => return None,
        };
        Some(Dependency { name, optional, source })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DependencySource {
    Version(String),
    Git {
        url: String,
        version: Option<String>,
        branch: Option<String>,
    },
}

fn single_key(doc: &TomlDoc, node: impl ast::KeyOwner) -> Option<Cow<str>> {
    let mut keys = node.keys(doc);
    let first = keys.next()?;
    if keys.next().is_some() {
        return None;
    }
    Some(first.name(doc))
}

fn string_value(doc: &TomlDoc, node: ast::Entry) -> Option<String> {
    match node.value(doc).kind(doc) {
        ast::ValueKind::StringLit(l) => Some(l.value(doc).into_owned()),
        _ => None
    }
}

fn bool_value(doc: &TomlDoc, node: ast::Entry) -> Option<bool> {
    match node.value(doc).kind(doc) {
        ast::ValueKind::Bool(l) => Some(l.value(doc)),
        _ => None
    }
}


#[cfg(test)]
mod tests;
