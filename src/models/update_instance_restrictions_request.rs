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
pub struct UpdateInstanceRestrictionsRequest {
	#[serde(
		rename = "allowlist",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub allowlist: Option<Option<bool>>,
	#[serde(
		rename = "blocklist",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub blocklist: Option<Option<bool>>,
	#[serde(
		rename = "block_email_subaddresses",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub block_email_subaddresses: Option<Option<bool>>,
}

impl UpdateInstanceRestrictionsRequest {
	pub fn new() -> UpdateInstanceRestrictionsRequest {
		UpdateInstanceRestrictionsRequest {
			allowlist: None,
			blocklist: None,
			block_email_subaddresses: None,
		}
	}
}
