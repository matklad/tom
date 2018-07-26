use tom::{TomlDoc, Item};

use Result;

#[derive(Debug, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub optional: bool,
    pub source: DependencySource,
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

pub struct CargoToml {
    doc: TomlDoc,
}

impl Dependency {
    fn from_item(name: String, item: &Item) -> Option<Dependency> {
        let mut optional = false;
        let source = match item {
            Item::String(s) => DependencySource::Version(s.clone()),
            Item::Map(map) => {
                let mut url = None;
                let mut branch = None;
                let mut version = None;

                for (k, v) in map.iter() {
                    match k.as_ref() {
                        "git" => url = Some(v.as_str()?.to_owned()),
                        "version" => version = Some(v.as_str()?.to_owned()),
                        "branch" => branch = Some(v.as_str()?.to_owned()),
                        "optional" => optional = v.as_bool()?,
                        _ => return None,
                    }
                }
                match url {
                    Some(url) => DependencySource::Git { url, version, branch },
                    None => match (version, branch) {
                        (Some(version), None) => DependencySource::Version(version),
                        _ => return None,
                    }
                }
            }
            _ => return None,
        };
        Some(Dependency { name, optional, source })
    }
}

impl CargoToml {
    pub fn new(text: &str) -> Result<CargoToml> {
        let mut doc = TomlDoc::new(text);
        if let Some(err) = doc.errors().first() {
            bail!(
                "syntax error at offset {}: {}",
                err.range().start(),
                err.message()
            );
        };

        doc.start_edit();
        Ok(CargoToml { doc })
    }

    pub fn text(&self) -> String {
        self.doc.cst().get_text(&self.doc)
    }

    pub fn dependencies(&self) -> Vec<Dependency> {
        let model = self.doc.model();
        let deps = match model.get("dependencies") {
            None => return Vec::new(),
            Some(deps) => match deps.as_map() {
                Some(map) => map,
                None => return Vec::new(),
            },
        };

        let deps = deps
            .iter()
            .flat_map(|(k, v)| Dependency::from_item(k.to_string(), v))
            .collect::<Vec<_>>();
        deps
    }

    pub fn update_dependency(&mut self, dep: &Dependency) -> Result<()> {
        let existing = self.dependencies().into_iter().find(|d| d.name == dep.name);
        match existing {
            None => self.insert_dep(dep),
            Some(d) => self.merge_deps(&d, dep),
        }
    }

    fn insert_dep(&mut self, dep: &Dependency) -> Result<()> {
        Ok(())
    }

    fn merge_deps(&mut self, old: &Dependency, new: &Dependency) -> Result<()> {
        Ok(())
    }
}
