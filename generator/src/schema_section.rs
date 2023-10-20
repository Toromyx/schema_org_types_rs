#[derive(Debug, Clone, Default)]
pub enum SchemaSection {
	#[default]
	General,
	Auto,
	Bib,
	HealthLifesci,
	Pending,
	Attic,
	Meta,
}

impl SchemaSection {
	pub fn feature_name(&self) -> &str {
		match self {
			SchemaSection::General => "general-schema-section",
			SchemaSection::Auto => "auto-schema-section",
			SchemaSection::Bib => "bib-schema-section",
			SchemaSection::HealthLifesci => "health-lifesci-schema-section",
			SchemaSection::Pending => "pending-schema-section",
			SchemaSection::Attic => "attic-schema-section",
			SchemaSection::Meta => "meta-schema-section",
		}
	}
}
