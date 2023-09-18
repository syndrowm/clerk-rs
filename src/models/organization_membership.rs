/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

/// OrganizationMembership : Hello world

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMembership {
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// String representing the object's type. Objects of the same type share the same value.
	#[serde(rename = "object", skip_serializing_if = "Option::is_none")]
	pub object: Option<Object>,
	#[serde(rename = "role", skip_serializing_if = "Option::is_none")]
	pub role: Option<Role>,
	/// Unix timestamp of creation.
	#[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
	pub created_at: Option<i64>,
	/// Unix timestamp of last update.
	#[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
	pub updated_at: Option<i64>,
	#[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
	pub organization: Option<Box<crate::models::Organization>>,
	#[serde(rename = "public_user_data", skip_serializing_if = "Option::is_none")]
	pub public_user_data: Option<Box<crate::models::OrganizationMembershipPublicUserData>>,
}

impl OrganizationMembership {
	/// Hello world
	pub fn new() -> OrganizationMembership {
		OrganizationMembership {
			id: None,
			object: None,
			role: None,
			created_at: None,
			updated_at: None,
			organization: None,
			public_user_data: None,
		}
	}
}

/// String representing the object's type. Objects of the same type share the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "organization_membership")]
	OrganizationMembership,
}

impl Default for Object {
	fn default() -> Object {
		Self::OrganizationMembership
	}
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
	#[serde(rename = "admin")]
	Admin,
	#[serde(rename = "basic_member")]
	BasicMember,
}

impl Default for Role {
	fn default() -> Role {
		Self::Admin
	}
}
