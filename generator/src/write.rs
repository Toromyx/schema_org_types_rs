use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Mutex, MutexGuard},
};

use derivative::Derivative;
use indicatif::{MultiProgress, ProgressBar};
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens};
use rayon::prelude::*;

use crate::{
    feature::Feature,
    schema::{
        class::Class, data_type::DataType, enumeration::Enumeration, property::Property, Schema,
    },
    schema_feature::SchemaFeature,
    sparql::{SchemaQueries, SchemaQuerySolution},
};

const IGNORED_SCHEMAS: &[&str] = &[
    "https://schema.org/True",
    "https://schema.org/False",
    "https://schema.org/DataType",
    "https://schema.org/Intangible",
    "https://schema.org/Series",
];

fn write_features(schema_features: Vec<SchemaFeature>) {
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
            schema_features
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
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct SchemaModuleInfo {
    pub name: String,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub feature: Feature,
}

impl<T: Schema> From<&T> for SchemaModuleInfo {
    fn from(value: &T) -> Self {
        Self {
            name: value.module_name(),
            feature: value.into(),
        }
    }
}

trait ToModuleString {
    fn to_module_string(&self) -> String;
}
impl ToModuleString for &[SchemaModuleInfo] {
    fn to_module_string(&self) -> String {
        let schema_mods_and_pub_uses = self.iter().map(|schema| {
            let feature_gate = schema.feature.feature_gate();
            let module_name = TokenStream::from_str(&format!("r#{}", schema.name)).unwrap();
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
    fn write_module(&self, schemas_dir: &Path);
    fn write_parent_module(schemas: &[SchemaModuleInfo], schemas_dir: &Path)
    where
        Self: Sized;
    fn write_parent_module_folder(schemas_dir: &Path);
}

trait HandleWrite {
    fn handle_write(
        store: &Store,
        solution: SchemaQuerySolution,
        schema_features: MutexGuard<BinaryHeap<SchemaFeature>>,
        schema_module_infos: MutexGuard<BinaryHeap<SchemaModuleInfo>>,
        schemas_dir: &Path,
    );
}

impl<T: Schema + ToTokens> HandleWrite for T {
    fn handle_write(
        store: &Store,
        solution: SchemaQuerySolution,
        mut schema_features: MutexGuard<BinaryHeap<SchemaFeature>>,
        mut schema_module_infos: MutexGuard<BinaryHeap<SchemaModuleInfo>>,
        schemas_dir: &Path,
    ) {
        let schema = Self::from_solution(store, solution);
        schema_features.push(SchemaFeature::from(&schema));
        schema_module_infos.push(SchemaModuleInfo::from(&schema));
        schema.write_module(schemas_dir);
    }
}

/// This function exists to make the futures below [`Send`].
///
/// Reference: <https://users.rust-lang.org/t/future-is-not-send-as-this-value-is-used-across-an-await-but-i-drop-the-value-before-the-await/57574/5>
fn pretty_please(str: &str) -> String {
    let syntax_tree = syn::parse_file(str).unwrap();
    prettyplease::unparse(&syntax_tree)
}

impl<T: Schema + ToTokens> WriteModules for T {
    fn write_module(&self, schemas_dir: &Path) {
        let mut file_path = PathBuf::from(&schemas_dir);
        file_path.push(Self::parent_module_name());
        file_path.push(format!("{}.rs", self.module_name()));
        std::fs::write(
            file_path,
            pretty_please(&self.to_token_stream().to_string()),
        )
        .unwrap();
    }

    fn write_parent_module(schema_module_infos: &[SchemaModuleInfo], schemas_dir: &Path) {
        let mut module_file = PathBuf::from(&schemas_dir);
        module_file.push(format!("{}.rs", Self::parent_module_name()));
        std::fs::write(
            module_file,
            pretty_please(&schema_module_infos.to_module_string()),
        )
        .unwrap();
    }

    fn write_parent_module_folder(schemas_dir: &Path) {
        let mut module_dir = PathBuf::from(&schemas_dir);
        module_dir.push(Self::parent_module_name());
        std::fs::create_dir(&module_dir).unwrap();
    }
}

pub fn write(store: &Store, multi_progress: &MultiProgress) {
    let schemas_dir = PathBuf::from("../src/schemas");
    std::fs::remove_dir_all(&schemas_dir).unwrap();
    std::fs::create_dir(&schemas_dir).unwrap();

    Class::write_parent_module_folder(&schemas_dir);
    Property::write_parent_module_folder(&schemas_dir);
    Enumeration::write_parent_module_folder(&schemas_dir);
    DataType::write_parent_module_folder(&schemas_dir);

    let schemas = store.get_schemas();

    let schema_features = Mutex::new(BinaryHeap::new());
    let class_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());
    let property_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());
    let enumeration_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());
    let data_types_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());

    let handle_class = |solution: SchemaQuerySolution| {
        Class::handle_write(
            store,
            solution,
            schema_features.lock().unwrap(),
            class_schema_module_infos.lock().unwrap(),
            &schemas_dir,
        );
    };

    let handle_property = |solution: SchemaQuerySolution| {
        Property::handle_write(
            store,
            solution,
            schema_features.lock().unwrap(),
            property_schema_module_infos.lock().unwrap(),
            &schemas_dir,
        );
    };

    let handle_enumeration = |solution: SchemaQuerySolution| {
        Enumeration::handle_write(
            store,
            solution,
            schema_features.lock().unwrap(),
            enumeration_schema_module_infos.lock().unwrap(),
            &schemas_dir,
        );
    };

    let handle_data_type = |solution: SchemaQuerySolution| {
        DataType::handle_write(
            store,
            solution,
            schema_features.lock().unwrap(),
            data_types_schema_module_infos.lock().unwrap(),
            &schemas_dir,
        );
    };

    let bar = multi_progress.add(ProgressBar::new(schemas.len() as u64));
    schemas.into_par_iter().for_each(|solution| {
        #[allow(clippy::never_loop)]
        loop {
            if IGNORED_SCHEMAS.contains(&solution.iri.as_str()) {
                break;
            }
            if store.is_enumeration_variant(&solution.iri) {
                break;
            }
            if store.is_property(&solution.iri) {
                handle_property(solution);
                break;
            }
            if store.is_data_type(&solution.iri) {
                handle_data_type(solution);
                break;
            }
            if store.is_enumeration(&solution.iri) {
                handle_enumeration(solution);
                break;
            }
            handle_class(solution);
            break;
        }
        bar.inc(1);
    });

    write_features(schema_features.into_inner().unwrap().into_sorted_vec());

    Class::write_parent_module(
        class_schema_module_infos
            .into_inner()
            .unwrap()
            .into_sorted_vec()
            .as_slice(),
        &schemas_dir,
    );
    Property::write_parent_module(
        property_schema_module_infos
            .into_inner()
            .unwrap()
            .into_sorted_vec()
            .as_slice(),
        &schemas_dir,
    );
    Enumeration::write_parent_module(
        enumeration_schema_module_infos
            .into_inner()
            .unwrap()
            .into_sorted_vec()
            .as_slice(),
        &schemas_dir,
    );
    DataType::write_parent_module(
        data_types_schema_module_infos
            .into_inner()
            .unwrap()
            .into_sorted_vec()
            .as_slice(),
        &schemas_dir,
    );
}
