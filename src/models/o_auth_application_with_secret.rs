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
pub struct OAuthApplicationWithSecret {
	#[serde(rename = "object")]
	pub object: Object,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "instance_id")]
	pub instance_id: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "client_id")]
	pub client_id: String,
	#[serde(rename = "public")]
	pub public: bool,
	#[serde(rename = "scopes")]
	pub scopes: String,
	#[serde(rename = "callback_url")]
	pub callback_url: String,
	#[serde(rename = "authorize_url")]
	pub authorize_url: String,
	#[serde(rename = "token_fetch_url")]
	pub token_fetch_url: String,
	#[serde(rename = "user_info_url")]
	pub user_info_url: String,
	/// Unix timestamp of creation.
	#[serde(rename = "created_at")]
	pub created_at: i64,
	/// Unix timestamp of last update.
	#[serde(rename = "updated_at")]
	pub updated_at: i64,
	/// Empty if public client.
	#[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
	pub client_secret: Option<String>,
}

impl OAuthApplicationWithSecret {
	pub fn new(
		object: Object,
		id: String,
		instance_id: String,
		name: String,
		client_id: String,
		public: bool,
		scopes: String,
		callback_url: String,
		authorize_url: String,
		token_fetch_url: String,
		user_info_url: String,
		created_at: i64,
		updated_at: i64,
	) -> OAuthApplicationWithSecret {
		OAuthApplicationWithSecret {
			object,
			id,
			instance_id,
			name,
			client_id,
			public,
			scopes,
			callback_url,
			authorize_url,
			token_fetch_url,
			user_info_url,
			created_at,
			updated_at,
			client_secret: None,
		}
	}
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "oauth_application")]
	OauthApplication,
}

impl Default for Object {
	fn default() -> Object {
		Self::OauthApplication
	}
}
