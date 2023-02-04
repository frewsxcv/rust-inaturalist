/*
 * iNaturalist API
 *
 * # https://api.inaturalist.org/v1/  [iNaturalist](https://www.inaturalist.org/) is a global community of naturalists, scientists, and members of the public sharing over a million wildlife sightings to teach one another about the natural world while creating high quality citizen science data for science and conservation. The iNaturalist technology infrastructure and open source software is administered by the [California Academy of Sciences](https://www.calacademy.org/) as part of their mission to explore, explain, and sustain life on Earth.  These API methods return data in JSON/JSONP and PNG response formats. They are meant to supplement the existing [iNaturalist API](https://www.inaturalist.org/pages/api+reference), implemented in Ruby on Rails, which has more functionality and supports more write operations, but tends to be slower and have less consistent response formats. Visit our [developers page](https://www.inaturalist.org/pages/developers) for more information. Write operations that expect and return JSON describe a single `body` parameter that represents the request body, which should be specified as JSON. See the \"Model\" of each body parameter for attributes that we accept in these JSON objects.  Multiple values for a single URL parameter should be separated by commas, e.g. `taxon_id=1,2,3`.  Map tiles are generated using the [node-mapnik](https://github.com/mapnik/node-mapnik) library, following the XYZ map tiling scheme. The \"Observation Tile\" methods accept nearly all the parameters of the observation search APIs, and will generate map tiles reflecting the same observations returned by searches. These \"Observation Tile\" methods have corresponding [UTFGrid](https://github.com/mapbox/utfgrid-spec) JSON responses which return information needed to make interactive maps.  Authentication in the Node API is handled via JSON Web Tokens (JWT). To obtain one, make an [OAuth-authenticated request](http://www.inaturalist.org/pages/api+reference#auth) to https://www.inaturalist.org/users/api_token. Each JWT will expire after 24 hours. Authentication required for all PUT and POST requests. Some GET requests will also include private information like hidden coordinates if the authenticated user has permission to view them.  iNaturalist Website: https://www.inaturalist.org/  Open Source Software: https://github.com/inaturalist/  ## Terms of Use  Use of this API is subject to the iNaturalist [Terms of Service](https://www.inaturalist.org/terms) and [Privacy Policy](https://www.inaturalist.org/privacy). We will block any use of our API that violates our Terms or Privacy Policy without notice. The API is intended to support application development, not data scraping. For pre- generated data exports, see https://www.inaturalist.org/pages/developers.  Please note that we throttle API usage to a max of 100 requests per minute, though we ask that you try to keep it to 60 requests per minute or lower, and to keep under 10,000 requests per day. If we notice usage that has serious impact on our performance we may institute blocks without notification.  Terms of Service: https://www.inaturalist.org/terms  Privacy Policy: https://www.inaturalist.org/privacy
 *
 * The version of the OpenAPI document: 1.3.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`observation_photos_id_delete`]
#[derive(Clone, Debug, Default)]
pub struct ObservationPhotosIdDeleteParams {
    /// ID of the record
    pub id: i32,
}

/// struct for passing parameters to the method [`observation_photos_id_put`]
#[derive(Clone, Debug, Default)]
pub struct ObservationPhotosIdPutParams {
    /// ID of the record
    pub id: i32,
    /// Position in which the photo is displayed for the observation
    pub observation_photo_left_square_bracket_position_right_square_bracket: Option<i32>,
    /// The photo
    pub file: Option<std::path::PathBuf>,
}

/// struct for passing parameters to the method [`observation_photos_post`]
#[derive(Clone, Debug, Default)]
pub struct ObservationPhotosPostParams {
    /// Observation ID
    pub observation_photo_left_square_bracket_observation_id_right_square_bracket: Option<i32>,
    /// Observation UUID
    pub observation_photo_left_square_bracket_uuid_right_square_bracket: Option<String>,
    /// The photo
    pub file: Option<std::path::PathBuf>,
}

/// struct for typed errors of method [`observation_photos_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObservationPhotosIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`observation_photos_id_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObservationPhotosIdPutError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`observation_photos_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObservationPhotosPostError {
    UnknownValue(serde_json::Value),
}

/// Delete an observation photo
pub async fn observation_photos_id_delete(
    configuration: &configuration::Configuration,
    params: ObservationPhotosIdDeleteParams,
) -> Result<(), Error<ObservationPhotosIdDeleteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/observation_photos/{id}",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ObservationPhotosIdDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an observation photo
pub async fn observation_photos_id_put(
    configuration: &configuration::Configuration,
    params: ObservationPhotosIdPutParams,
) -> Result<(), Error<ObservationPhotosIdPutError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let observation_photo_left_square_bracket_position_right_square_bracket =
        params.observation_photo_left_square_bracket_position_right_square_bracket;
    let file = params.file;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/observation_photos/{id}",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) =
        observation_photo_left_square_bracket_position_right_square_bracket
    {
        local_var_form = local_var_form.text(
            "observation_photo[position]",
            local_var_param_value.to_string(),
        );
    }
    // TODO: support file upload for 'file' parameter
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ObservationPhotosIdPutError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create an observation photo
pub async fn observation_photos_post(
    configuration: &configuration::Configuration,
    params: ObservationPhotosPostParams,
) -> Result<(), Error<ObservationPhotosPostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let observation_photo_left_square_bracket_observation_id_right_square_bracket =
        params.observation_photo_left_square_bracket_observation_id_right_square_bracket;
    let observation_photo_left_square_bracket_uuid_right_square_bracket =
        params.observation_photo_left_square_bracket_uuid_right_square_bracket;
    let file = params.file;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/observation_photos", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) =
        observation_photo_left_square_bracket_observation_id_right_square_bracket
    {
        local_var_form = local_var_form.text(
            "observation_photo[observation_id]",
            local_var_param_value.to_string(),
        );
    }
    if let Some(local_var_param_value) =
        observation_photo_left_square_bracket_uuid_right_square_bracket
    {
        local_var_form =
            local_var_form.text("observation_photo[uuid]", local_var_param_value.to_string());
    }
    // TODO: support file upload for 'file' parameter
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ObservationPhotosPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
