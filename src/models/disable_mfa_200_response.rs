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
pub struct DisableMfa200Response {
	#[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
	pub user_id: Option<String>,
}

impl DisableMfa200Response {
	pub fn new() -> DisableMfa200Response {
		DisableMfa200Response { user_id: None }
	}
}
