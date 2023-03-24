/*
 * iNaturalist API
 *
 * # https://api.inaturalist.org/v1/  [iNaturalist](https://www.inaturalist.org/) is a global community of naturalists, scientists, and members of the public sharing over a million wildlife sightings to teach one another about the natural world while creating high quality citizen science data for science and conservation. The iNaturalist technology infrastructure and open source software is administered by the [California Academy of Sciences](https://www.calacademy.org/) as part of their mission to explore, explain, and sustain life on Earth.  These API methods return data in JSON/JSONP and PNG response formats. They are meant to supplement the existing [iNaturalist API](https://www.inaturalist.org/pages/api+reference), implemented in Ruby on Rails, which has more functionality and supports more write operations, but tends to be slower and have less consistent response formats. Visit our [developers page](https://www.inaturalist.org/pages/developers) for more information. Write operations that expect and return JSON describe a single `body` parameter that represents the request body, which should be specified as JSON. See the \"Model\" of each body parameter for attributes that we accept in these JSON objects.  Multiple values for a single URL parameter should be separated by commas, e.g. `taxon_id=1,2,3`.  Map tiles are generated using the [node-mapnik](https://github.com/mapnik/node-mapnik) library, following the XYZ map tiling scheme. The \"Observation Tile\" methods accept nearly all the parameters of the observation search APIs, and will generate map tiles reflecting the same observations returned by searches. These \"Observation Tile\" methods have corresponding [UTFGrid](https://github.com/mapbox/utfgrid-spec) JSON responses which return information needed to make interactive maps.  Authentication in the Node API is handled via JSON Web Tokens (JWT). To obtain one, make an [OAuth-authenticated request](http://www.inaturalist.org/pages/api+reference#auth) to https://www.inaturalist.org/users/api_token. Each JWT will expire after 24 hours. Authentication required for all PUT and POST requests. Some GET requests will also include private information like hidden coordinates if the authenticated user has permission to view them.  iNaturalist Website: https://www.inaturalist.org/  Open Source Software: https://github.com/inaturalist/  ## Terms of Use  Use of this API is subject to the iNaturalist [Terms of Service](https://www.inaturalist.org/terms) and [Privacy Policy](https://www.inaturalist.org/privacy). We will block any use of our API that violates our Terms or Privacy Policy without notice. The API is intended to support application development, not data scraping. For pre- generated data exports, see https://www.inaturalist.org/pages/developers.  Please note that we throttle API usage to a max of 100 requests per minute, though we ask that you try to keep it to 60 requests per minute or lower, and to keep under 10,000 requests per day. If we notice usage that has serious impact on our performance we may institute blocks without notification.  Terms of Service: https://www.inaturalist.org/terms  Privacy Policy: https://www.inaturalist.org/privacy
 *
 * The version of the OpenAPI document: 1.3.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ObservationTaxon {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "iconic_taxon_id", skip_serializing_if = "Option::is_none")]
    pub iconic_taxon_id: Option<i32>,
    #[serde(rename = "iconic_taxon_name", skip_serializing_if = "Option::is_none")]
    pub iconic_taxon_name: Option<String>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "preferred_common_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_common_name: Option<String>,
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    #[serde(rename = "rank_level", skip_serializing_if = "Option::is_none")]
    pub rank_level: Option<f32>,
    #[serde(rename = "ancestor_ids", skip_serializing_if = "Option::is_none")]
    pub ancestor_ids: Option<Vec<i32>>,
    #[serde(rename = "ancestry", skip_serializing_if = "Option::is_none")]
    pub ancestry: Option<String>,
    #[serde(
        rename = "conservation_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub conservation_status: Option<Box<crate::models::RawConservationStatus>>,
    #[serde(rename = "endemic", skip_serializing_if = "Option::is_none")]
    pub endemic: Option<bool>,
    #[serde(
        rename = "establishment_means",
        skip_serializing_if = "Option::is_none"
    )]
    pub establishment_means: Option<Box<crate::models::EstablishmentMeans>>,
    #[serde(rename = "introduced", skip_serializing_if = "Option::is_none")]
    pub introduced: Option<bool>,
    #[serde(rename = "native", skip_serializing_if = "Option::is_none")]
    pub native: Option<bool>,
    #[serde(rename = "threatened", skip_serializing_if = "Option::is_none")]
    pub threatened: Option<bool>,
}

impl ObservationTaxon {
    pub fn new() -> ObservationTaxon {
        ObservationTaxon {
            id: None,
            iconic_taxon_id: None,
            iconic_taxon_name: None,
            is_active: None,
            name: None,
            preferred_common_name: None,
            rank: None,
            rank_level: None,
            ancestor_ids: None,
            ancestry: None,
            conservation_status: None,
            endemic: None,
            establishment_means: None,
            introduced: None,
            native: None,
            threatened: None,
        }
    }
}
