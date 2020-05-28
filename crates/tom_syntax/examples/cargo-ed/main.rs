//! FIXME: write short doc here

// extern crate clap;
// #[macro_use]
// extern crate failure;
// extern crate itertools;
// extern crate tom;

// mod cargo_toml;

// use std::{fs};
// use clap::{App, Arg};
// pub use cargo_toml::{CargoToml, Dependency, DependencySource};

// type Result<T> = ::std::result::Result<T, failure::Error>;

// fn main() -> Result<()> {
//     let app = App::new("cargo-ed")
//         .arg(Arg::with_name("package").required(true))
//         .arg(
//             Arg::with_name("manifest-path")
//                 .takes_value(true)
//                 .long("manifest-path"),
//         )
//         .arg(
//             Arg::with_name("version")
//                 .takes_value(true)
//                 .long("version")
//                 .required_unless("git"),
//         )
//         .arg(Arg::with_name("git").takes_value(true).long("git"))
//         .arg(
//             Arg::with_name("branch")
//                 .takes_value(true)
//                 .long("branch")
//                 .requires("git"),
//         )
//         .arg(Arg::with_name("optional").long("optional"));
//     let matches = app.get_matches();
//     let manifest_path = matches.value_of("manifest-path").unwrap_or("Cargo.toml");
// //    let dep = Dependency {
// //        name: matches.value_of("package").unwrap().to_string(),
// //        optional: matches.is_present("optional"),
// //        source: match matches.value_of("git") {
// //            Some(url) => DependencySource::Git {
// //                url: url.to_string(),
// //                version: matches.value_of("version").map(|s| s.to_string()),
// //                branch: matches.value_of("branch").map(|s| s.to_string()),
// //            },
// //            None => DependencySource::Version(matches.value_of("version").unwrap().to_string()),
// //        },
// //    };
// //
//     let text = fs::read_to_string(manifest_path)?;
//     let toml = CargoToml::new(&text)?;
//     println!("{:?   }", toml.dependencies());
// //    toml.update_dependency(&dep)?;
// //    let result = toml.text();
// //
// //    fs::write(manifest_path, result)?;
// //
//     Ok(())
// }

// #[cfg(test)]
// mod tests;

fn main() {}
