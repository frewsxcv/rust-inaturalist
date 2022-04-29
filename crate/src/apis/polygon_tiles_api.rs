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

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`places_place_id_zoom_xy_png_get`]
#[derive(Clone, Debug, Default)]
pub struct PlacesPlaceIdZoomXyPngGetParams {
    /// Place ID
    pub place_id: i32,
    /// Zoom level. Z coordinate in the XYZ tiling scheme
    pub zoom: i32,
    /// X coordinate in the XYZ tiling scheme. Must be less than 2^zoom
    pub x: i32,
    /// Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom
    pub y: i32,
    /// Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers 
    pub ttl: Option<String>
}

/// struct for passing parameters to the method [`taxon_places_taxon_id_zoom_xy_png_get`]
#[derive(Clone, Debug, Default)]
pub struct TaxonPlacesTaxonIdZoomXyPngGetParams {
    /// Taxon ID
    pub taxon_id: i32,
    /// Zoom level. Z coordinate in the XYZ tiling scheme
    pub zoom: i32,
    /// X coordinate in the XYZ tiling scheme. Must be less than 2^zoom
    pub x: i32,
    /// Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom
    pub y: i32,
    /// Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers 
    pub ttl: Option<String>
}

/// struct for passing parameters to the method [`taxon_ranges_taxon_id_zoom_xy_png_get`]
#[derive(Clone, Debug, Default)]
pub struct TaxonRangesTaxonIdZoomXyPngGetParams {
    /// Taxon ID
    pub taxon_id: i32,
    /// Zoom level. Z coordinate in the XYZ tiling scheme
    pub zoom: i32,
    /// X coordinate in the XYZ tiling scheme. Must be less than 2^zoom
    pub x: i32,
    /// Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom
    pub y: i32,
    /// Primary color to use in tile creation. Accepts common colors by string (e.g. `color=blue`), and accepts escaped color HEX codes (e.g. `color=%2386a91c`) 
    pub color: Option<String>,
    /// Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers 
    pub ttl: Option<String>
}


/// struct for typed errors of method [`places_place_id_zoom_xy_png_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlacesPlaceIdZoomXyPngGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`taxon_places_taxon_id_zoom_xy_png_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaxonPlacesTaxonIdZoomXyPngGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`taxon_ranges_taxon_id_zoom_xy_png_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaxonRangesTaxonIdZoomXyPngGetError {
    UnknownValue(serde_json::Value),
}


/// Returns a PNG map tile representing the boundary of this place, following the XYZ tiling scheme 
pub async fn places_place_id_zoom_xy_png_get(configuration: &configuration::Configuration, params: PlacesPlaceIdZoomXyPngGetParams) -> Result<(), Error<PlacesPlaceIdZoomXyPngGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let place_id = params.place_id;
    let zoom = params.zoom;
    let x = params.x;
    let y = params.y;
    let ttl = params.ttl;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/places/{place_id}/{zoom}/{x}/{y}.png", local_var_configuration.base_path, place_id=place_id, zoom=zoom, x=x, y=y);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ttl {
        local_var_req_builder = local_var_req_builder.query(&[("ttl", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PlacesPlaceIdZoomXyPngGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a PNG map tile representing the boundaries of places this taxon is known to occur, following the XYZ tiling scheme 
pub async fn taxon_places_taxon_id_zoom_xy_png_get(configuration: &configuration::Configuration, params: TaxonPlacesTaxonIdZoomXyPngGetParams) -> Result<(), Error<TaxonPlacesTaxonIdZoomXyPngGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let taxon_id = params.taxon_id;
    let zoom = params.zoom;
    let x = params.x;
    let y = params.y;
    let ttl = params.ttl;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/taxon_places/{taxon_id}/{zoom}/{x}/{y}.png", local_var_configuration.base_path, taxon_id=taxon_id, zoom=zoom, x=x, y=y);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ttl {
        local_var_req_builder = local_var_req_builder.query(&[("ttl", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<TaxonPlacesTaxonIdZoomXyPngGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a PNG map tile representing the range of this taxon, following the XYZ tiling scheme 
pub async fn taxon_ranges_taxon_id_zoom_xy_png_get(configuration: &configuration::Configuration, params: TaxonRangesTaxonIdZoomXyPngGetParams) -> Result<(), Error<TaxonRangesTaxonIdZoomXyPngGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let taxon_id = params.taxon_id;
    let zoom = params.zoom;
    let x = params.x;
    let y = params.y;
    let color = params.color;
    let ttl = params.ttl;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/taxon_ranges/{taxon_id}/{zoom}/{x}/{y}.png", local_var_configuration.base_path, taxon_id=taxon_id, zoom=zoom, x=x, y=y);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = color {
        local_var_req_builder = local_var_req_builder.query(&[("color", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ttl {
        local_var_req_builder = local_var_req_builder.query(&[("ttl", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<TaxonRangesTaxonIdZoomXyPngGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

