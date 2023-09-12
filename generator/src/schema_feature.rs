use derivative::Derivative;

use crate::schema::Schema;

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaFeature {
    schema_feature_name: String,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    child_feature_names: Vec<String>,
}

impl SchemaFeature {
    pub fn to_toml_representation(&self) -> String {
        format!(
            r#"{} = [
{}]
"#,
            self.schema_feature_name,
            self.child_feature_names
                .iter()
                .map(|child_feature_name| format!("    \"{}\",\n", child_feature_name))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl<T: Schema> From<&T> for SchemaFeature {
    fn from(value: &T) -> Self {
        Self {
            schema_feature_name: value.feature_name(),
            child_feature_names: value.child_feature_names(),
        }
    }
}
