/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettings {
	/// String representing the object's type. Objects of the same type share the same value.
	#[serde(rename = "object")]
	pub object: Object,
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "max_allowed_memberships")]
	pub max_allowed_memberships: i32,
	/// The default for whether an admin can delete an organization with the Frontend API.
	#[serde(rename = "admin_delete_enabled", skip_serializing_if = "Option::is_none")]
	pub admin_delete_enabled: Option<bool>,
}

impl OrganizationSettings {
	pub fn new(object: Object, enabled: bool, max_allowed_memberships: i32) -> OrganizationSettings {
		OrganizationSettings {
			object,
			enabled,
			max_allowed_memberships,
			admin_delete_enabled: None,
		}
	}
}

/// String representing the object's type. Objects of the same type share the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "organization_settings")]
	OrganizationSettings,
}

impl Default for Object {
	fn default() -> Object {
		Self::OrganizationSettings
	}
}
