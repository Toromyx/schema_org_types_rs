use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use convert_case::{Case, Casing};
use indicatif::{MultiProgress, ProgressBar};
use quote::{__private::TokenStream, quote, ToTokens};
use rayon::prelude::*;

use crate::{
    feature_gate::FeatureGate,
    schema::{
        class::Class, data_type::DataType, enumeration::Enumeration, property::Property, Schema,
    },
    Schemas,
};

pub fn write_features(schemas: &Schemas, multi_progress: &MultiProgress) {
    let features = schemas.to_features(multi_progress);
    let cargo_toml_path = PathBuf::from("../Cargo.toml");
    let mut cargo_toml_content: Vec<u8> = {
        let cargo_toml_file = File::open(&cargo_toml_path).unwrap();
        let reader = BufReader::new(&cargo_toml_file);
        reader
            .lines()
            .map(|line_result| line_result.unwrap())
            .take_while(|line| line != "# generated features")
            .flat_map(|line| format!("{}\n", line).into_bytes())
            .collect()
    };
    {
        cargo_toml_content.extend("# generated features\n".to_string().into_bytes());
        cargo_toml_content.extend(
            features
                .iter()
                .flat_map(|feature| feature.to_toml_representation().into_bytes())
                .collect::<Vec<u8>>(),
        );
        let mut cargo_toml_file = File::create(&cargo_toml_path).unwrap();
        cargo_toml_file
            .write_all(cargo_toml_content.as_slice())
            .unwrap();
    }
}

trait ToModuleString {
    fn to_module_string(&self) -> String;
}

impl<T: Schema> ToModuleString for &[T] {
    fn to_module_string(&self) -> String {
        let schema_mods_and_pub_uses = self.iter().map(|schema| {
            let feature_gate = schema.feature_gate();
            let module_name =
                TokenStream::from_str(&format!("r#{}", schema.name().to_case(Case::Snake)))
                    .unwrap();
            quote!(
                #feature_gate
                mod #module_name;
                #feature_gate
                pub use #module_name::*;
            )
        });
        quote!(
            use super::*;
            #(#schema_mods_and_pub_uses)*
        )
        .to_string()
    }
}

trait WriteModules {
    fn write_modules(schemas: &[Self], schemas_dir: &Path, multi_progress: &MultiProgress)
    where
        Self: Sized;
    fn write_module(schemas: &[Self], schemas_dir: &Path)
    where
        Self: Sized;
}

/// This function exists to make the futures below [`Send`].
///
/// Reference: <https://users.rust-lang.org/t/future-is-not-send-as-this-value-is-used-across-an-await-but-i-drop-the-value-before-the-await/57574/5>
fn pretty_please(str: &str) -> String {
    let syntax_tree = syn::parse_file(str).unwrap();
    prettyplease::unparse(&syntax_tree)
}

impl<T: Schema + ToTokens + Sync> WriteModules for T {
    fn write_modules(schemas: &[Self], schemas_dir: &Path, multi_progress: &MultiProgress) {
        let mut module_dir = PathBuf::from(&schemas_dir);
        module_dir.push(Self::module_name());
        std::fs::create_dir(&module_dir).unwrap();
        let bar = multi_progress.add(ProgressBar::new(schemas.len() as u64));
        schemas.par_iter().for_each(|schema| {
            let mut file_path = PathBuf::from(&module_dir);
            file_path.push(format!("{}.rs", schema.name().to_case(Case::Snake)));
            std::fs::write(
                file_path,
                pretty_please(&schema.to_token_stream().to_string()),
            )
            .unwrap();
            bar.inc(1);
        });
    }

    fn write_module(schemas: &[Self], schemas_dir: &Path) {
        let mut module_file = PathBuf::from(&schemas_dir);
        module_file.push(format!("{}.rs", Self::module_name()));
        std::fs::write(module_file, pretty_please(&schemas.to_module_string())).unwrap();
    }
}

pub fn write_schemas(schemas: &Schemas, multi_progress: &MultiProgress) {
    let schemas_dir = PathBuf::from("../src/schemas");
    std::fs::remove_dir_all(&schemas_dir).unwrap();
    std::fs::create_dir(&schemas_dir).unwrap();

    Class::write_modules(&schemas.classes, &schemas_dir, multi_progress);
    Class::write_module(&schemas.classes, &schemas_dir);
    Property::write_modules(&schemas.properties, &schemas_dir, multi_progress);
    Property::write_module(&schemas.properties, &schemas_dir);
    Enumeration::write_modules(&schemas.enumerations, &schemas_dir, multi_progress);
    Enumeration::write_module(&schemas.enumerations, &schemas_dir);
    DataType::write_modules(&schemas.data_types, &schemas_dir, multi_progress);
    DataType::write_module(&schemas.data_types, &schemas_dir);
}
