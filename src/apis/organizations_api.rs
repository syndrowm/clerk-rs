/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_organization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrganizationError {
    Status400(crate::models::ClerkErrors),
    Status403(crate::models::ClerkErrors),
    Status422(crate::models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_organization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrganizationError {
    Status404(crate::models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_organization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrganizationError {
    Status403(crate::models::ClerkErrors),
    Status404(crate::models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_organizations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrganizationsError {
    Status400(crate::models::ClerkErrors),
    Status403(crate::models::ClerkErrors),
    Status422(crate::models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`merge_organization_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MergeOrganizationMetadataError {
    Status400(crate::models::ClerkErrors),
    Status401(crate::models::ClerkErrors),
    Status404(crate::models::ClerkErrors),
    Status422(crate::models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_organization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrganizationError {
    Status404(crate::models::ClerkErrors),
    Status422(crate::models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_organization_logo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadOrganizationLogoError {
    Status400(crate::models::ClerkErrors),
    Status403(crate::models::ClerkErrors),
    Status404(crate::models::ClerkErrors),
    Status413(crate::models::ClerkErrors),
    UnknownValue(serde_json::Value),
}


/// Creates a new organization with the given name for an instance. In order to successfully create an organization you need to provide the ID of the User who will become the organization administrator. You can specify an optional slug for the new organization. If provided, the organization slug can contain only lowercase alphanumeric characters (letters and digits) and the dash \"-\". Organization slugs must be unique for the instance. You can provide additional metadata for the organization and set any custom attribute you want. Organizations support private and public metadata. Private metadata can only be accessed from the Backend API. Public metadata can be accessed from the Backend API, and are read-only from the Frontend API.
pub async fn create_organization(clerk_configuration: &configuration::ClerkConfiguration, create_organization_request: Option<crate::models::CreateOrganizationRequest>) -> Result<crate::models::Organization, Error<CreateOrganizationError>> {
    let local_var_configuration = clerk_configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organizations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_organization_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateOrganizationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes the given organization. Please note that deleting an organization will also delete all memberships and invitations. This is not reversible.
pub async fn delete_organization(clerk_configuration: &configuration::ClerkConfiguration, organization_id: &str) -> Result<crate::models::DeletedObject, Error<DeleteOrganizationError>> {
    let local_var_configuration = clerk_configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organizations/{organization_id}", local_var_configuration.base_path, organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteOrganizationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetches the organization whose ID or slug matches the provided `id_or_slug` URL query parameter.
pub async fn get_organization(clerk_configuration: &configuration::ClerkConfiguration, organization_id: &str) -> Result<crate::models::Organization, Error<GetOrganizationError>> {
    let local_var_configuration = clerk_configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organizations/{organization_id}", local_var_configuration.base_path, organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetOrganizationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This request returns the list of organizations for an instance. Results can be paginated using the optional `limit` and `offset` query parameters. The organizations are ordered by descending creation date. Most recent organizations will be returned first.
pub async fn list_organizations(clerk_configuration: &configuration::ClerkConfiguration, limit: Option<f32>, offset: Option<f32>, include_members_count: Option<bool>, query: Option<&str>) -> Result<crate::models::Organizations, Error<ListOrganizationsError>> {
    let local_var_configuration = clerk_configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organizations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_members_count {
        local_var_req_builder = local_var_req_builder.query(&[("include_members_count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = query {
        local_var_req_builder = local_var_req_builder.query(&[("query", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListOrganizationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update organization metadata attributes by merging existing values with the provided parameters. Metadata values will be updated via a deep merge. Deep meaning that any nested JSON objects will be merged as well. You can remove metadata keys at any level by setting their value to `null`.
pub async fn merge_organization_metadata(clerk_configuration: &configuration::ClerkConfiguration, organization_id: &str, merge_organization_metadata_request: crate::models::MergeOrganizationMetadataRequest) -> Result<crate::models::Organization, Error<MergeOrganizationMetadataError>> {
    let local_var_configuration = clerk_configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organizations/{organization_id}/metadata", local_var_configuration.base_path, organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&merge_organization_metadata_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MergeOrganizationMetadataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates an existing organization
pub async fn update_organization(clerk_configuration: &configuration::ClerkConfiguration, organization_id: &str, update_organization_request: crate::models::UpdateOrganizationRequest) -> Result<crate::models::Organization, Error<UpdateOrganizationError>> {
    let local_var_configuration = clerk_configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organizations/{organization_id}", local_var_configuration.base_path, organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_organization_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateOrganizationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set or replace an organization's logo, by uploading an image file. This endpoint uses the `multipart/form-data` request content type and accepts a file of image type. The file size cannot exceed 10MB. Only the following file content types are supported: `image/jpeg`, `image/png`, `image/gif`, `image/webp`, `image/x-icon`, `image/vnd.microsoft.icon`.
pub async fn upload_organization_logo(clerk_configuration: &configuration::ClerkConfiguration, organization_id: &str, uploader_user_id: Option<&str>, file: Option<std::path::PathBuf>) -> Result<crate::models::OrganizationWithLogo, Error<UploadOrganizationLogoError>> {
    let local_var_configuration = clerk_configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organizations/{organization_id}/logo", local_var_configuration.base_path, organization_id=crate::apis::urlencode(organization_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) = uploader_user_id {
        local_var_form = local_var_form.text("uploader_user_id", local_var_param_value.to_string());
    }
    // TODO: support file upload for 'file' parameter
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UploadOrganizationLogoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

