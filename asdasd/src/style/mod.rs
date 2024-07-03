pub mod attributes;
pub mod layout;
pub mod types;

#[derive(Clone, PartialEq, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Stylesheet {
	pub normal:		attributes::StyleAttributes,
	pub hovered:	attributes::StyleAttributes,
	pub focused:	attributes::StyleAttributes,
}