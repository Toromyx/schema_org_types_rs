use std::{
	collections::BinaryHeap,
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
	schema::{
		class::Class, data_type::DataType, enumeration::Enumeration, property::Property, Schema,
	},
	sparql::{node_type::NodeType, SchemaQueries, SchemaQuerySolution},
};

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct SchemaModuleInfo {
	pub name: String,
}

impl<T: Schema> From<&T> for SchemaModuleInfo {
	fn from(value: &T) -> Self {
		Self {
			name: value.module_name(),
		}
	}
}

trait ToModuleString {
	fn to_module_string(&self) -> String;
}
impl ToModuleString for &[SchemaModuleInfo] {
	fn to_module_string(&self) -> String {
		let schema_mods_and_pub_uses = self.iter().map(|schema| {
			let module_name = TokenStream::from_str(&format!("r#{}", schema.name)).unwrap();
			quote!(
				mod #module_name;
				pub use self::#module_name::*;
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
		schema_module_infos: MutexGuard<BinaryHeap<SchemaModuleInfo>>,
		schemas_dir: &Path,
	);
}

impl<T: Schema + ToTokens> HandleWrite for T {
	fn handle_write(
		store: &Store,
		solution: SchemaQuerySolution,
		mut schema_module_infos: MutexGuard<BinaryHeap<SchemaModuleInfo>>,
		schemas_dir: &Path,
	) {
		let schema = Self::from_solution(store, solution);
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

	let class_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());
	let property_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());
	let enumeration_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());
	let data_types_schema_module_infos = Mutex::new(BinaryHeap::<SchemaModuleInfo>::new());

	let handle_class = |solution: SchemaQuerySolution| {
		Class::handle_write(
			store,
			solution,
			class_schema_module_infos.lock().unwrap(),
			&schemas_dir,
		);
	};

	let handle_property = |solution: SchemaQuerySolution| {
		Property::handle_write(
			store,
			solution,
			property_schema_module_infos.lock().unwrap(),
			&schemas_dir,
		);
	};

	let handle_enumeration = |solution: SchemaQuerySolution| {
		Enumeration::handle_write(
			store,
			solution,
			enumeration_schema_module_infos.lock().unwrap(),
			&schemas_dir,
		);
	};

	let handle_data_type = |solution: SchemaQuerySolution| {
		DataType::handle_write(
			store,
			solution,
			data_types_schema_module_infos.lock().unwrap(),
			&schemas_dir,
		);
	};

	let bar = multi_progress.add(ProgressBar::new(schemas.len() as u64));
	schemas.into_par_iter().for_each(|solution| {
		match NodeType::from_iri(store, &solution.iri) {
			NodeType::EnumerationVariant => {}
			NodeType::Property => {
				handle_property(solution);
			}
			NodeType::DataType => {
				handle_data_type(solution);
			}
			NodeType::Enumeration => {
				handle_enumeration(solution);
			}
			NodeType::Class => {
				handle_class(solution);
			}
		};
		bar.inc(1);
	});

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
