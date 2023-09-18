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
pub struct CreateInvitationRequest {
	/// The email address the invitation will be sent to
	#[serde(rename = "email_address")]
	pub email_address: String,
	/// Metadata that will be attached to the newly created invitation. The value of this property should be a well-formed JSON object. Once the user accepts the invitation and signs up, these metadata will end up in the user's public metadata.
	#[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
	pub public_metadata: Option<serde_json::Value>,
	/// Optional URL which specifies where to redirect the user once they click the invitation link. This is only required if you have implemented a [custom flow](https://clerk.com/docs/authentication/invitations#custom-flow) and you're not using Clerk Hosted Pages or Clerk Components.
	#[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
	pub redirect_url: Option<String>,
}

impl CreateInvitationRequest {
	pub fn new(email_address: String) -> CreateInvitationRequest {
		CreateInvitationRequest {
			email_address,
			public_metadata: None,
			redirect_url: None,
		}
	}
}
