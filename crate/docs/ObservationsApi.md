# \ObservationsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**observations_deleted_get**](ObservationsApi.md#observations_deleted_get) | **GET** /observations/deleted | Observations Deleted
[**observations_get**](ObservationsApi.md#observations_get) | **GET** /observations | Observation Search
[**observations_histogram_get**](ObservationsApi.md#observations_histogram_get) | **GET** /observations/histogram | Observation Histogram
[**observations_id_delete**](ObservationsApi.md#observations_id_delete) | **DELETE** /observations/{id} | Observation Delete
[**observations_id_fave_post**](ObservationsApi.md#observations_id_fave_post) | **POST** /observations/{id}/fave | Observations Fave
[**observations_id_get**](ObservationsApi.md#observations_id_get) | **GET** /observations/{id} | Observation Details
[**observations_id_put**](ObservationsApi.md#observations_id_put) | **PUT** /observations/{id} | Observation Update
[**observations_id_quality_metric_delete**](ObservationsApi.md#observations_id_quality_metric_delete) | **DELETE** /observations/{id}/quality/{metric} | Quality Metric Delete
[**observations_id_quality_metric_post**](ObservationsApi.md#observations_id_quality_metric_post) | **POST** /observations/{id}/quality/{metric} | Quality Metric Set
[**observations_id_review_post**](ObservationsApi.md#observations_id_review_post) | **POST** /observations/{id}/review | Observations Review
[**observations_id_subscriptions_get**](ObservationsApi.md#observations_id_subscriptions_get) | **GET** /observations/{id}/subscriptions | Observation Subscriptions
[**observations_id_taxon_summary_get**](ObservationsApi.md#observations_id_taxon_summary_get) | **GET** /observations/{id}/taxon_summary | Observation Taxon Summary
[**observations_id_unfave_delete**](ObservationsApi.md#observations_id_unfave_delete) | **DELETE** /observations/{id}/unfave | Observations Unfave
[**observations_id_unreview_post**](ObservationsApi.md#observations_id_unreview_post) | **POST** /observations/{id}/unreview | Observations Unreview
[**observations_id_viewed_updates_put**](ObservationsApi.md#observations_id_viewed_updates_put) | **PUT** /observations/{id}/viewed_updates | Observation Field Value Update
[**observations_identifiers_get**](ObservationsApi.md#observations_identifiers_get) | **GET** /observations/identifiers | Observation Identifiers
[**observations_observers_get**](ObservationsApi.md#observations_observers_get) | **GET** /observations/observers | Observation Observers
[**observations_popular_field_values_get**](ObservationsApi.md#observations_popular_field_values_get) | **GET** /observations/popular_field_values | Observation Popular Field Values
[**observations_post**](ObservationsApi.md#observations_post) | **POST** /observations | Observation Create
[**observations_species_counts_get**](ObservationsApi.md#observations_species_counts_get) | **GET** /observations/species_counts | Observation Species Counts
[**observations_updates_get**](ObservationsApi.md#observations_updates_get) | **GET** /observations/updates | Observation User Updates
[**subscriptions_observation_id_subscribe_post**](ObservationsApi.md#subscriptions_observation_id_subscribe_post) | **POST** /subscriptions/observation/{id}/subscribe | Observation Subscribe
[**votes_unvote_observation_id_delete**](ObservationsApi.md#votes_unvote_observation_id_delete) | **DELETE** /votes/unvote/observation/{id} | Observation Unvote
[**votes_vote_observation_id_post**](ObservationsApi.md#votes_vote_observation_id_post) | **POST** /votes/vote/observation/{id} | Observation Vote



## observations_deleted_get

> observations_deleted_get(since)
Observations Deleted

Given a starting date, return an array of IDs of the authenticated user's observations that have been deleted since that date. Requires authentication 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | **String** | Deleted at or after this time | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_get

> crate::models::ObservationsResponse observations_get(acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, day, month, year, term_id, term_value_id, without_term_id, without_term_value_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, q, search_on, quality_grade, updated_since, viewer_id, reviewed, locale, preferred_place_id, ttl, page, per_page, order, order_by, only_id)
Observation Search

Given zero to many of following parameters, returns observations matching the search criteria. The large size of the observations index prevents us from supporting the `page` parameter when retrieving records from large result sets. If you need to retrieve large numbers of records, use the `per_page` and `id_above` or `id_below` parameters instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acc** | Option<**bool**> | Whether or not positional accuracy / coordinate uncertainty has been specified |  |
**captive** | Option<**bool**> | Captive or cultivated observations |  |
**endemic** | Option<**bool**> | Observations whose taxa are endemic to their location |  |
**geo** | Option<**bool**> | Observations that are georeferenced |  |
**id_please** | Option<**bool**> | Observations with the deprecated `ID, Please!` flag. Note that this will return observations, but that this attribute is no longer used. |  |
**identified** | Option<**bool**> | Observations that have community identifications |  |
**introduced** | Option<**bool**> | Observations whose taxa are introduced in their location  |  |
**mappable** | Option<**bool**> | Observations that show on map tiles |  |
**native** | Option<**bool**> | Observations whose taxa are native to their location |  |
**out_of_range** | Option<**bool**> | Observations whose taxa are outside their known ranges |  |
**pcid** | Option<**bool**> | Observations identified by the curator of a project. If the `project_id` parameter is also specified, this will only consider observations identified by curators of the specified project(s)  |  |
**photos** | Option<**bool**> | Observations with photos |  |
**popular** | Option<**bool**> | Observations that have been favorited by at least one user  |  |
**sounds** | Option<**bool**> | Observations with sounds |  |
**taxon_is_active** | Option<**bool**> | Observations of active taxon concepts  |  |
**threatened** | Option<**bool**> | Observations whose taxa are threatened in their location  |  |
**verifiable** | Option<**bool**> | Observations with a `quality_grade` of either `needs_id` or `research`. Equivalent to `quality_grade=needs_id,research`  |  |
**licensed** | Option<**bool**> | License attribute of an observation must not be null |  |
**photo_licensed** | Option<**bool**> | License attribute of at least one photo of an observation must not be null |  |
**id** | Option<[**Vec<String>**](String.md)> | Must have this ID |  |
**not_id** | Option<[**Vec<String>**](String.md)> | Must not have this ID |  |
**license** | Option<[**Vec<String>**](String.md)> | Observation must have this license |  |
**ofv_datatype** | Option<[**Vec<String>**](String.md)> | Must have an observation field value with this datatype |  |
**photo_license** | Option<[**Vec<String>**](String.md)> | Must have at least one photo with this license |  |
**place_id** | Option<[**Vec<i32>**](i32.md)> | Must be observed within the place with this ID |  |
**project_id** | Option<[**Vec<String>**](String.md)> | Must be added to the project this ID or slug |  |
**rank** | Option<[**Vec<String>**](String.md)> | Taxon must have this rank |  |
**site_id** | Option<[**Vec<String>**](String.md)> | Must be affiliated with the iNaturalist network website with this ID  |  |
**sound_license** | Option<[**Vec<String>**](String.md)> | Must have at least one sound with this license |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | Only show observations of these taxa and their descendants |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude observations of these taxa and their descendants |  |
**taxon_name** | Option<[**Vec<String>**](String.md)> | Taxon must have a scientific or common name matching this string  |  |
**user_id** | Option<[**Vec<String>**](String.md)> | User must have this ID or login |  |
**user_login** | Option<[**Vec<String>**](String.md)> | User must have this login |  |
**ident_user_id** | Option<**i32**> | Observations identified by a particular user |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**acc_above** | Option<**String**> | Must have a positional accuracy above this value (meters) |  |
**acc_below** | Option<**String**> | Must have a positional accuracy below this value (meters) |  |
**acc_below_or_unknown** | Option<**String**> | Positional accuracy must be below this value (in meters) or be unknown |  |
**d1** | Option<**String**> | Must be observed on or after this date |  |
**d2** | Option<**String**> | Must be observed on or before this date |  |
**created_d1** | Option<**String**> | Must be created at or after this time |  |
**created_d2** | Option<**String**> | Must be created at or before this time |  |
**created_on** | Option<**String**> | Must be created on this date |  |
**observed_on** | Option<**String**> | Must be observed on this date |  |
**unobserved_by_user_id** | Option<**i32**> | Must not be of a taxon previously observed by this user |  |
**apply_project_rules_for** | Option<**String**> | Must match the rules of the project with this ID or slug |  |
**cs** | Option<**String**> | Taxon must have this conservation status code. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csa** | Option<**String**> | Taxon must have a conservation status from this authority. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csi** | Option<[**Vec<String>**](String.md)> | Taxon must have this IUCN conservation status. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**geoprivacy** | Option<[**Vec<String>**](String.md)> | Must have this geoprivacy setting |  |
**taxon_geoprivacy** | Option<[**Vec<String>**](String.md)> | Filter observations by the most conservative geoprivacy applied by a conservation status associated with one of the taxa proposed in the current identifications.  |  |
**hrank** | Option<**String**> | Taxon must have this rank or lower |  |
**lrank** | Option<**String**> | Taxon must have this rank or higher |  |
**iconic_taxa** | Option<[**Vec<String>**](String.md)> | Taxon must by within this iconic taxon |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**identifications** | Option<**String**> | Identifications must meet these criteria |  |
**lat** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**lng** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**radius** | Option<**String**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**nelat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**nelng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**list_id** | Option<**i32**> | Taxon must be in the list with this ID |  |
**not_in_project** | Option<**String**> | Must not be in the project with this ID or slug |  |
**not_matching_project_rules_for** | Option<**String**> | Must not match the rules of the project with this ID or slug |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |
**locale** | Option<**String**> | Locale preference for taxon common names  |  |
**preferred_place_id** | Option<**i32**> | Place preference for regional taxon common names  |  |
**ttl** | Option<**String**> | Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers  |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**order** | Option<**String**> | Sort order |  |[default to desc]
**order_by** | Option<**String**> | Sort field |  |[default to created_at]
**only_id** | Option<**bool**> | Return only the record IDs |  |

### Return type

[**crate::models::ObservationsResponse**](ObservationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_histogram_get

> observations_histogram_get(acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, day, month, year, term_id, term_value_id, without_term_id, without_term_value_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, q, search_on, quality_grade, updated_since, viewer_id, reviewed, locale, preferred_place_id, ttl, date_field, interval)
Observation Histogram

Given zero to many of following parameters, returns histogram data about observations matching the search criteria 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acc** | Option<**bool**> | Whether or not positional accuracy / coordinate uncertainty has been specified |  |
**captive** | Option<**bool**> | Captive or cultivated observations |  |
**endemic** | Option<**bool**> | Observations whose taxa are endemic to their location |  |
**geo** | Option<**bool**> | Observations that are georeferenced |  |
**id_please** | Option<**bool**> | Observations with the deprecated `ID, Please!` flag. Note that this will return observations, but that this attribute is no longer used. |  |
**identified** | Option<**bool**> | Observations that have community identifications |  |
**introduced** | Option<**bool**> | Observations whose taxa are introduced in their location  |  |
**mappable** | Option<**bool**> | Observations that show on map tiles |  |
**native** | Option<**bool**> | Observations whose taxa are native to their location |  |
**out_of_range** | Option<**bool**> | Observations whose taxa are outside their known ranges |  |
**pcid** | Option<**bool**> | Observations identified by the curator of a project. If the `project_id` parameter is also specified, this will only consider observations identified by curators of the specified project(s)  |  |
**photos** | Option<**bool**> | Observations with photos |  |
**popular** | Option<**bool**> | Observations that have been favorited by at least one user  |  |
**sounds** | Option<**bool**> | Observations with sounds |  |
**taxon_is_active** | Option<**bool**> | Observations of active taxon concepts  |  |
**threatened** | Option<**bool**> | Observations whose taxa are threatened in their location  |  |
**verifiable** | Option<**bool**> | Observations with a `quality_grade` of either `needs_id` or `research`. Equivalent to `quality_grade=needs_id,research`  |  |
**licensed** | Option<**bool**> | License attribute of an observation must not be null |  |
**photo_licensed** | Option<**bool**> | License attribute of at least one photo of an observation must not be null |  |
**id** | Option<[**Vec<String>**](String.md)> | Must have this ID |  |
**not_id** | Option<[**Vec<String>**](String.md)> | Must not have this ID |  |
**license** | Option<[**Vec<String>**](String.md)> | Observation must have this license |  |
**ofv_datatype** | Option<[**Vec<String>**](String.md)> | Must have an observation field value with this datatype |  |
**photo_license** | Option<[**Vec<String>**](String.md)> | Must have at least one photo with this license |  |
**place_id** | Option<[**Vec<i32>**](i32.md)> | Must be observed within the place with this ID |  |
**project_id** | Option<[**Vec<String>**](String.md)> | Must be added to the project this ID or slug |  |
**rank** | Option<[**Vec<String>**](String.md)> | Taxon must have this rank |  |
**site_id** | Option<[**Vec<String>**](String.md)> | Must be affiliated with the iNaturalist network website with this ID  |  |
**sound_license** | Option<[**Vec<String>**](String.md)> | Must have at least one sound with this license |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | Only show observations of these taxa and their descendants |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude observations of these taxa and their descendants |  |
**taxon_name** | Option<[**Vec<String>**](String.md)> | Taxon must have a scientific or common name matching this string  |  |
**user_id** | Option<[**Vec<String>**](String.md)> | User must have this ID or login |  |
**user_login** | Option<[**Vec<String>**](String.md)> | User must have this login |  |
**ident_user_id** | Option<**i32**> | Observations identified by a particular user |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**acc_above** | Option<**String**> | Must have a positional accuracy above this value (meters) |  |
**acc_below** | Option<**String**> | Must have a positional accuracy below this value (meters) |  |
**acc_below_or_unknown** | Option<**String**> | Positional accuracy must be below this value (in meters) or be unknown |  |
**d1** | Option<**String**> | Must be observed on or after this date |  |
**d2** | Option<**String**> | Must be observed on or before this date |  |
**created_d1** | Option<**String**> | Must be created at or after this time |  |
**created_d2** | Option<**String**> | Must be created at or before this time |  |
**created_on** | Option<**String**> | Must be created on this date |  |
**observed_on** | Option<**String**> | Must be observed on this date |  |
**unobserved_by_user_id** | Option<**i32**> | Must not be of a taxon previously observed by this user |  |
**apply_project_rules_for** | Option<**String**> | Must match the rules of the project with this ID or slug |  |
**cs** | Option<**String**> | Taxon must have this conservation status code. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csa** | Option<**String**> | Taxon must have a conservation status from this authority. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csi** | Option<[**Vec<String>**](String.md)> | Taxon must have this IUCN conservation status. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**geoprivacy** | Option<[**Vec<String>**](String.md)> | Must have this geoprivacy setting |  |
**taxon_geoprivacy** | Option<[**Vec<String>**](String.md)> | Filter observations by the most conservative geoprivacy applied by a conservation status associated with one of the taxa proposed in the current identifications.  |  |
**hrank** | Option<**String**> | Taxon must have this rank or lower |  |
**lrank** | Option<**String**> | Taxon must have this rank or higher |  |
**iconic_taxa** | Option<[**Vec<String>**](String.md)> | Taxon must by within this iconic taxon |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**identifications** | Option<**String**> | Identifications must meet these criteria |  |
**lat** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**lng** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**radius** | Option<**String**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**nelat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**nelng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**list_id** | Option<**i32**> | Taxon must be in the list with this ID |  |
**not_in_project** | Option<**String**> | Must not be in the project with this ID or slug |  |
**not_matching_project_rules_for** | Option<**String**> | Must not match the rules of the project with this ID or slug |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |
**locale** | Option<**String**> | Locale preference for taxon common names  |  |
**preferred_place_id** | Option<**i32**> | Place preference for regional taxon common names  |  |
**ttl** | Option<**String**> | Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers  |  |
**date_field** | Option<**String**> | Histogram basis: when the observation was created or observed  |  |[default to observed]
**interval** | Option<**String**> | Time interval for histogram, with groups starting on or contained within the group value. The year, month, week, day, and hour options will set default values for `d1` or `created_d1` depending on the value of `date_field`, to limit the number of groups returned. You can override those values if you want data from a longer or shorter time span. The `hour` interval only works with `date_field=created`, and this you should filter dates with `created_d[1,2]`  |  |[default to month_of_year]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_delete

> observations_id_delete(id)
Observation Delete

Delete an observation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_fave_post

> observations_id_fave_post(id)
Observations Fave

Fave an observation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_get

> crate::models::ObservationsShowResponse observations_id_get(id)
Observation Details

Given an ID, or an array of IDs in comma-delimited format, returns corresponding observations. A maximum of 200 results will be returned 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<i32>**](i32.md) | Must have this ID | [required] |

### Return type

[**crate::models::ObservationsShowResponse**](ObservationsShowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_put

> observations_id_put(id, body)
Observation Update

Update an observation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PostObservation**](PostObservation.md)> | Comment object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_quality_metric_delete

> observations_id_quality_metric_delete(id, metric)
Quality Metric Delete

Delete a quality metric 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**metric** | **String** | Data quality category | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_quality_metric_post

> observations_id_quality_metric_post(id, metric, body)
Quality Metric Set

Set the value of a quality metric 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**metric** | **String** | Data quality category | [required] |
**body** | Option<[**PostQuality**](PostQuality.md)> | Quality object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_review_post

> observations_id_review_post(id)
Observations Review

Review an observation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_subscriptions_get

> observations_id_subscriptions_get(id)
Observation Subscriptions

Fetches any subscriptions the current user has to this observation or the observer 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_taxon_summary_get

> observations_id_taxon_summary_get(id)
Observation Taxon Summary

Fetches information about this observation's taxon, within the context of this observation's location 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_unfave_delete

> observations_id_unfave_delete(id)
Observations Unfave

Unfave an observation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_unreview_post

> observations_id_unreview_post(id)
Observations Unreview

Unreview an observation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_id_viewed_updates_put

> observations_id_viewed_updates_put(id)
Observation Field Value Update

Mark all updates associated with this observation as viewed by logged-in user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_identifiers_get

> crate::models::UserCountsResponse observations_identifiers_get(acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, day, month, year, term_id, term_value_id, without_term_id, without_term_value_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, q, search_on, quality_grade, updated_since, viewer_id, reviewed, locale, preferred_place_id, ttl)
Observation Identifiers

Given zero to many of following parameters, returns identifiers of observations matching the search criteria and the count of observations they have identified, ordered by count descending. A maximum of 500 results will be returned 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acc** | Option<**bool**> | Whether or not positional accuracy / coordinate uncertainty has been specified |  |
**captive** | Option<**bool**> | Captive or cultivated observations |  |
**endemic** | Option<**bool**> | Observations whose taxa are endemic to their location |  |
**geo** | Option<**bool**> | Observations that are georeferenced |  |
**id_please** | Option<**bool**> | Observations with the deprecated `ID, Please!` flag. Note that this will return observations, but that this attribute is no longer used. |  |
**identified** | Option<**bool**> | Observations that have community identifications |  |
**introduced** | Option<**bool**> | Observations whose taxa are introduced in their location  |  |
**mappable** | Option<**bool**> | Observations that show on map tiles |  |
**native** | Option<**bool**> | Observations whose taxa are native to their location |  |
**out_of_range** | Option<**bool**> | Observations whose taxa are outside their known ranges |  |
**pcid** | Option<**bool**> | Observations identified by the curator of a project. If the `project_id` parameter is also specified, this will only consider observations identified by curators of the specified project(s)  |  |
**photos** | Option<**bool**> | Observations with photos |  |
**popular** | Option<**bool**> | Observations that have been favorited by at least one user  |  |
**sounds** | Option<**bool**> | Observations with sounds |  |
**taxon_is_active** | Option<**bool**> | Observations of active taxon concepts  |  |
**threatened** | Option<**bool**> | Observations whose taxa are threatened in their location  |  |
**verifiable** | Option<**bool**> | Observations with a `quality_grade` of either `needs_id` or `research`. Equivalent to `quality_grade=needs_id,research`  |  |
**licensed** | Option<**bool**> | License attribute of an observation must not be null |  |
**photo_licensed** | Option<**bool**> | License attribute of at least one photo of an observation must not be null |  |
**id** | Option<[**Vec<String>**](String.md)> | Must have this ID |  |
**not_id** | Option<[**Vec<String>**](String.md)> | Must not have this ID |  |
**license** | Option<[**Vec<String>**](String.md)> | Observation must have this license |  |
**ofv_datatype** | Option<[**Vec<String>**](String.md)> | Must have an observation field value with this datatype |  |
**photo_license** | Option<[**Vec<String>**](String.md)> | Must have at least one photo with this license |  |
**place_id** | Option<[**Vec<i32>**](i32.md)> | Must be observed within the place with this ID |  |
**project_id** | Option<[**Vec<String>**](String.md)> | Must be added to the project this ID or slug |  |
**rank** | Option<[**Vec<String>**](String.md)> | Taxon must have this rank |  |
**site_id** | Option<[**Vec<String>**](String.md)> | Must be affiliated with the iNaturalist network website with this ID  |  |
**sound_license** | Option<[**Vec<String>**](String.md)> | Must have at least one sound with this license |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | Only show observations of these taxa and their descendants |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude observations of these taxa and their descendants |  |
**taxon_name** | Option<[**Vec<String>**](String.md)> | Taxon must have a scientific or common name matching this string  |  |
**user_id** | Option<[**Vec<String>**](String.md)> | User must have this ID or login |  |
**user_login** | Option<[**Vec<String>**](String.md)> | User must have this login |  |
**ident_user_id** | Option<**i32**> | Observations identified by a particular user |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**acc_above** | Option<**String**> | Must have a positional accuracy above this value (meters) |  |
**acc_below** | Option<**String**> | Must have a positional accuracy below this value (meters) |  |
**acc_below_or_unknown** | Option<**String**> | Positional accuracy must be below this value (in meters) or be unknown |  |
**d1** | Option<**String**> | Must be observed on or after this date |  |
**d2** | Option<**String**> | Must be observed on or before this date |  |
**created_d1** | Option<**String**> | Must be created at or after this time |  |
**created_d2** | Option<**String**> | Must be created at or before this time |  |
**created_on** | Option<**String**> | Must be created on this date |  |
**observed_on** | Option<**String**> | Must be observed on this date |  |
**unobserved_by_user_id** | Option<**i32**> | Must not be of a taxon previously observed by this user |  |
**apply_project_rules_for** | Option<**String**> | Must match the rules of the project with this ID or slug |  |
**cs** | Option<**String**> | Taxon must have this conservation status code. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csa** | Option<**String**> | Taxon must have a conservation status from this authority. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csi** | Option<[**Vec<String>**](String.md)> | Taxon must have this IUCN conservation status. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**geoprivacy** | Option<[**Vec<String>**](String.md)> | Must have this geoprivacy setting |  |
**taxon_geoprivacy** | Option<[**Vec<String>**](String.md)> | Filter observations by the most conservative geoprivacy applied by a conservation status associated with one of the taxa proposed in the current identifications.  |  |
**hrank** | Option<**String**> | Taxon must have this rank or lower |  |
**lrank** | Option<**String**> | Taxon must have this rank or higher |  |
**iconic_taxa** | Option<[**Vec<String>**](String.md)> | Taxon must by within this iconic taxon |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**identifications** | Option<**String**> | Identifications must meet these criteria |  |
**lat** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**lng** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**radius** | Option<**String**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**nelat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**nelng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**list_id** | Option<**i32**> | Taxon must be in the list with this ID |  |
**not_in_project** | Option<**String**> | Must not be in the project with this ID or slug |  |
**not_matching_project_rules_for** | Option<**String**> | Must not match the rules of the project with this ID or slug |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |
**locale** | Option<**String**> | Locale preference for taxon common names  |  |
**preferred_place_id** | Option<**i32**> | Place preference for regional taxon common names  |  |
**ttl** | Option<**String**> | Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers  |  |

### Return type

[**crate::models::UserCountsResponse**](UserCountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_observers_get

> crate::models::ObservationsObserversResponse observations_observers_get(acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, day, month, year, term_id, term_value_id, without_term_id, without_term_value_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, q, search_on, quality_grade, updated_since, viewer_id, reviewed, locale, preferred_place_id, ttl)
Observation Observers

Given zero to many of following parameters, returns observers of observations matching the search criteria and the count of observations and distinct taxa of rank `species` they have observed. A maximum of 500 results will be returned 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acc** | Option<**bool**> | Whether or not positional accuracy / coordinate uncertainty has been specified |  |
**captive** | Option<**bool**> | Captive or cultivated observations |  |
**endemic** | Option<**bool**> | Observations whose taxa are endemic to their location |  |
**geo** | Option<**bool**> | Observations that are georeferenced |  |
**id_please** | Option<**bool**> | Observations with the deprecated `ID, Please!` flag. Note that this will return observations, but that this attribute is no longer used. |  |
**identified** | Option<**bool**> | Observations that have community identifications |  |
**introduced** | Option<**bool**> | Observations whose taxa are introduced in their location  |  |
**mappable** | Option<**bool**> | Observations that show on map tiles |  |
**native** | Option<**bool**> | Observations whose taxa are native to their location |  |
**out_of_range** | Option<**bool**> | Observations whose taxa are outside their known ranges |  |
**pcid** | Option<**bool**> | Observations identified by the curator of a project. If the `project_id` parameter is also specified, this will only consider observations identified by curators of the specified project(s)  |  |
**photos** | Option<**bool**> | Observations with photos |  |
**popular** | Option<**bool**> | Observations that have been favorited by at least one user  |  |
**sounds** | Option<**bool**> | Observations with sounds |  |
**taxon_is_active** | Option<**bool**> | Observations of active taxon concepts  |  |
**threatened** | Option<**bool**> | Observations whose taxa are threatened in their location  |  |
**verifiable** | Option<**bool**> | Observations with a `quality_grade` of either `needs_id` or `research`. Equivalent to `quality_grade=needs_id,research`  |  |
**licensed** | Option<**bool**> | License attribute of an observation must not be null |  |
**photo_licensed** | Option<**bool**> | License attribute of at least one photo of an observation must not be null |  |
**id** | Option<[**Vec<String>**](String.md)> | Must have this ID |  |
**not_id** | Option<[**Vec<String>**](String.md)> | Must not have this ID |  |
**license** | Option<[**Vec<String>**](String.md)> | Observation must have this license |  |
**ofv_datatype** | Option<[**Vec<String>**](String.md)> | Must have an observation field value with this datatype |  |
**photo_license** | Option<[**Vec<String>**](String.md)> | Must have at least one photo with this license |  |
**place_id** | Option<[**Vec<i32>**](i32.md)> | Must be observed within the place with this ID |  |
**project_id** | Option<[**Vec<String>**](String.md)> | Must be added to the project this ID or slug |  |
**rank** | Option<[**Vec<String>**](String.md)> | Taxon must have this rank |  |
**site_id** | Option<[**Vec<String>**](String.md)> | Must be affiliated with the iNaturalist network website with this ID  |  |
**sound_license** | Option<[**Vec<String>**](String.md)> | Must have at least one sound with this license |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | Only show observations of these taxa and their descendants |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude observations of these taxa and their descendants |  |
**taxon_name** | Option<[**Vec<String>**](String.md)> | Taxon must have a scientific or common name matching this string  |  |
**user_id** | Option<[**Vec<String>**](String.md)> | User must have this ID or login |  |
**user_login** | Option<[**Vec<String>**](String.md)> | User must have this login |  |
**ident_user_id** | Option<**i32**> | Observations identified by a particular user |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**acc_above** | Option<**String**> | Must have a positional accuracy above this value (meters) |  |
**acc_below** | Option<**String**> | Must have a positional accuracy below this value (meters) |  |
**acc_below_or_unknown** | Option<**String**> | Positional accuracy must be below this value (in meters) or be unknown |  |
**d1** | Option<**String**> | Must be observed on or after this date |  |
**d2** | Option<**String**> | Must be observed on or before this date |  |
**created_d1** | Option<**String**> | Must be created at or after this time |  |
**created_d2** | Option<**String**> | Must be created at or before this time |  |
**created_on** | Option<**String**> | Must be created on this date |  |
**observed_on** | Option<**String**> | Must be observed on this date |  |
**unobserved_by_user_id** | Option<**i32**> | Must not be of a taxon previously observed by this user |  |
**apply_project_rules_for** | Option<**String**> | Must match the rules of the project with this ID or slug |  |
**cs** | Option<**String**> | Taxon must have this conservation status code. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csa** | Option<**String**> | Taxon must have a conservation status from this authority. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csi** | Option<[**Vec<String>**](String.md)> | Taxon must have this IUCN conservation status. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**geoprivacy** | Option<[**Vec<String>**](String.md)> | Must have this geoprivacy setting |  |
**taxon_geoprivacy** | Option<[**Vec<String>**](String.md)> | Filter observations by the most conservative geoprivacy applied by a conservation status associated with one of the taxa proposed in the current identifications.  |  |
**hrank** | Option<**String**> | Taxon must have this rank or lower |  |
**lrank** | Option<**String**> | Taxon must have this rank or higher |  |
**iconic_taxa** | Option<[**Vec<String>**](String.md)> | Taxon must by within this iconic taxon |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**identifications** | Option<**String**> | Identifications must meet these criteria |  |
**lat** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**lng** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**radius** | Option<**String**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**nelat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**nelng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**list_id** | Option<**i32**> | Taxon must be in the list with this ID |  |
**not_in_project** | Option<**String**> | Must not be in the project with this ID or slug |  |
**not_matching_project_rules_for** | Option<**String**> | Must not match the rules of the project with this ID or slug |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |
**locale** | Option<**String**> | Locale preference for taxon common names  |  |
**preferred_place_id** | Option<**i32**> | Place preference for regional taxon common names  |  |
**ttl** | Option<**String**> | Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers  |  |

### Return type

[**crate::models::ObservationsObserversResponse**](ObservationsObserversResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_popular_field_values_get

> observations_popular_field_values_get(acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, day, month, year, term_id, term_value_id, without_term_id, without_term_value_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, q, search_on, quality_grade, updated_since, viewer_id, reviewed, locale, preferred_place_id, ttl)
Observation Popular Field Values

Given zero to many of following parameters, returns an array of relevant controlled terms values and a monthly histogram 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acc** | Option<**bool**> | Whether or not positional accuracy / coordinate uncertainty has been specified |  |
**captive** | Option<**bool**> | Captive or cultivated observations |  |
**endemic** | Option<**bool**> | Observations whose taxa are endemic to their location |  |
**geo** | Option<**bool**> | Observations that are georeferenced |  |
**id_please** | Option<**bool**> | Observations with the deprecated `ID, Please!` flag. Note that this will return observations, but that this attribute is no longer used. |  |
**identified** | Option<**bool**> | Observations that have community identifications |  |
**introduced** | Option<**bool**> | Observations whose taxa are introduced in their location  |  |
**mappable** | Option<**bool**> | Observations that show on map tiles |  |
**native** | Option<**bool**> | Observations whose taxa are native to their location |  |
**out_of_range** | Option<**bool**> | Observations whose taxa are outside their known ranges |  |
**pcid** | Option<**bool**> | Observations identified by the curator of a project. If the `project_id` parameter is also specified, this will only consider observations identified by curators of the specified project(s)  |  |
**photos** | Option<**bool**> | Observations with photos |  |
**popular** | Option<**bool**> | Observations that have been favorited by at least one user  |  |
**sounds** | Option<**bool**> | Observations with sounds |  |
**taxon_is_active** | Option<**bool**> | Observations of active taxon concepts  |  |
**threatened** | Option<**bool**> | Observations whose taxa are threatened in their location  |  |
**verifiable** | Option<**bool**> | Observations with a `quality_grade` of either `needs_id` or `research`. Equivalent to `quality_grade=needs_id,research`  |  |
**licensed** | Option<**bool**> | License attribute of an observation must not be null |  |
**photo_licensed** | Option<**bool**> | License attribute of at least one photo of an observation must not be null |  |
**id** | Option<[**Vec<String>**](String.md)> | Must have this ID |  |
**not_id** | Option<[**Vec<String>**](String.md)> | Must not have this ID |  |
**license** | Option<[**Vec<String>**](String.md)> | Observation must have this license |  |
**ofv_datatype** | Option<[**Vec<String>**](String.md)> | Must have an observation field value with this datatype |  |
**photo_license** | Option<[**Vec<String>**](String.md)> | Must have at least one photo with this license |  |
**place_id** | Option<[**Vec<i32>**](i32.md)> | Must be observed within the place with this ID |  |
**project_id** | Option<[**Vec<String>**](String.md)> | Must be added to the project this ID or slug |  |
**rank** | Option<[**Vec<String>**](String.md)> | Taxon must have this rank |  |
**site_id** | Option<[**Vec<String>**](String.md)> | Must be affiliated with the iNaturalist network website with this ID  |  |
**sound_license** | Option<[**Vec<String>**](String.md)> | Must have at least one sound with this license |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | Only show observations of these taxa and their descendants |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude observations of these taxa and their descendants |  |
**taxon_name** | Option<[**Vec<String>**](String.md)> | Taxon must have a scientific or common name matching this string  |  |
**user_id** | Option<[**Vec<String>**](String.md)> | User must have this ID or login |  |
**user_login** | Option<[**Vec<String>**](String.md)> | User must have this login |  |
**ident_user_id** | Option<**i32**> | Observations identified by a particular user |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**acc_above** | Option<**String**> | Must have a positional accuracy above this value (meters) |  |
**acc_below** | Option<**String**> | Must have a positional accuracy below this value (meters) |  |
**acc_below_or_unknown** | Option<**String**> | Positional accuracy must be below this value (in meters) or be unknown |  |
**d1** | Option<**String**> | Must be observed on or after this date |  |
**d2** | Option<**String**> | Must be observed on or before this date |  |
**created_d1** | Option<**String**> | Must be created at or after this time |  |
**created_d2** | Option<**String**> | Must be created at or before this time |  |
**created_on** | Option<**String**> | Must be created on this date |  |
**observed_on** | Option<**String**> | Must be observed on this date |  |
**unobserved_by_user_id** | Option<**i32**> | Must not be of a taxon previously observed by this user |  |
**apply_project_rules_for** | Option<**String**> | Must match the rules of the project with this ID or slug |  |
**cs** | Option<**String**> | Taxon must have this conservation status code. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csa** | Option<**String**> | Taxon must have a conservation status from this authority. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csi** | Option<[**Vec<String>**](String.md)> | Taxon must have this IUCN conservation status. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**geoprivacy** | Option<[**Vec<String>**](String.md)> | Must have this geoprivacy setting |  |
**taxon_geoprivacy** | Option<[**Vec<String>**](String.md)> | Filter observations by the most conservative geoprivacy applied by a conservation status associated with one of the taxa proposed in the current identifications.  |  |
**hrank** | Option<**String**> | Taxon must have this rank or lower |  |
**lrank** | Option<**String**> | Taxon must have this rank or higher |  |
**iconic_taxa** | Option<[**Vec<String>**](String.md)> | Taxon must by within this iconic taxon |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**identifications** | Option<**String**> | Identifications must meet these criteria |  |
**lat** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**lng** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**radius** | Option<**String**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**nelat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**nelng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**list_id** | Option<**i32**> | Taxon must be in the list with this ID |  |
**not_in_project** | Option<**String**> | Must not be in the project with this ID or slug |  |
**not_matching_project_rules_for** | Option<**String**> | Must not match the rules of the project with this ID or slug |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |
**locale** | Option<**String**> | Locale preference for taxon common names  |  |
**preferred_place_id** | Option<**i32**> | Place preference for regional taxon common names  |  |
**ttl** | Option<**String**> | Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_post

> observations_post(body)
Observation Create

Create an observation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostObservation**](PostObservation.md)> | Comment object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_species_counts_get

> crate::models::SpeciesCountsResponse observations_species_counts_get(acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, day, month, year, term_id, term_value_id, without_term_id, without_term_value_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, q, search_on, quality_grade, updated_since, viewer_id, reviewed, locale, preferred_place_id, ttl)
Observation Species Counts

Given zero to many of following parameters, returns `leaf taxa` associated with observations matching the search criteria and the count of observations they are associated with, ordered by count descending. `Leaf taxa` are the leaves of the taxonomic tree containing only the taxa associated with observations matching the search criteria. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acc** | Option<**bool**> | Whether or not positional accuracy / coordinate uncertainty has been specified |  |
**captive** | Option<**bool**> | Captive or cultivated observations |  |
**endemic** | Option<**bool**> | Observations whose taxa are endemic to their location |  |
**geo** | Option<**bool**> | Observations that are georeferenced |  |
**id_please** | Option<**bool**> | Observations with the deprecated `ID, Please!` flag. Note that this will return observations, but that this attribute is no longer used. |  |
**identified** | Option<**bool**> | Observations that have community identifications |  |
**introduced** | Option<**bool**> | Observations whose taxa are introduced in their location  |  |
**mappable** | Option<**bool**> | Observations that show on map tiles |  |
**native** | Option<**bool**> | Observations whose taxa are native to their location |  |
**out_of_range** | Option<**bool**> | Observations whose taxa are outside their known ranges |  |
**pcid** | Option<**bool**> | Observations identified by the curator of a project. If the `project_id` parameter is also specified, this will only consider observations identified by curators of the specified project(s)  |  |
**photos** | Option<**bool**> | Observations with photos |  |
**popular** | Option<**bool**> | Observations that have been favorited by at least one user  |  |
**sounds** | Option<**bool**> | Observations with sounds |  |
**taxon_is_active** | Option<**bool**> | Observations of active taxon concepts  |  |
**threatened** | Option<**bool**> | Observations whose taxa are threatened in their location  |  |
**verifiable** | Option<**bool**> | Observations with a `quality_grade` of either `needs_id` or `research`. Equivalent to `quality_grade=needs_id,research`  |  |
**licensed** | Option<**bool**> | License attribute of an observation must not be null |  |
**photo_licensed** | Option<**bool**> | License attribute of at least one photo of an observation must not be null |  |
**id** | Option<[**Vec<String>**](String.md)> | Must have this ID |  |
**not_id** | Option<[**Vec<String>**](String.md)> | Must not have this ID |  |
**license** | Option<[**Vec<String>**](String.md)> | Observation must have this license |  |
**ofv_datatype** | Option<[**Vec<String>**](String.md)> | Must have an observation field value with this datatype |  |
**photo_license** | Option<[**Vec<String>**](String.md)> | Must have at least one photo with this license |  |
**place_id** | Option<[**Vec<i32>**](i32.md)> | Must be observed within the place with this ID |  |
**project_id** | Option<[**Vec<String>**](String.md)> | Must be added to the project this ID or slug |  |
**rank** | Option<[**Vec<String>**](String.md)> | Taxon must have this rank |  |
**site_id** | Option<[**Vec<String>**](String.md)> | Must be affiliated with the iNaturalist network website with this ID  |  |
**sound_license** | Option<[**Vec<String>**](String.md)> | Must have at least one sound with this license |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | Only show observations of these taxa and their descendants |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude observations of these taxa and their descendants |  |
**taxon_name** | Option<[**Vec<String>**](String.md)> | Taxon must have a scientific or common name matching this string  |  |
**user_id** | Option<[**Vec<String>**](String.md)> | User must have this ID or login |  |
**user_login** | Option<[**Vec<String>**](String.md)> | User must have this login |  |
**ident_user_id** | Option<**i32**> | Observations identified by a particular user |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**acc_above** | Option<**String**> | Must have a positional accuracy above this value (meters) |  |
**acc_below** | Option<**String**> | Must have a positional accuracy below this value (meters) |  |
**acc_below_or_unknown** | Option<**String**> | Positional accuracy must be below this value (in meters) or be unknown |  |
**d1** | Option<**String**> | Must be observed on or after this date |  |
**d2** | Option<**String**> | Must be observed on or before this date |  |
**created_d1** | Option<**String**> | Must be created at or after this time |  |
**created_d2** | Option<**String**> | Must be created at or before this time |  |
**created_on** | Option<**String**> | Must be created on this date |  |
**observed_on** | Option<**String**> | Must be observed on this date |  |
**unobserved_by_user_id** | Option<**i32**> | Must not be of a taxon previously observed by this user |  |
**apply_project_rules_for** | Option<**String**> | Must match the rules of the project with this ID or slug |  |
**cs** | Option<**String**> | Taxon must have this conservation status code. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csa** | Option<**String**> | Taxon must have a conservation status from this authority. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**csi** | Option<[**Vec<String>**](String.md)> | Taxon must have this IUCN conservation status. If the `place_id` parameter is also specified, this will only consider statuses specific to that place  |  |
**geoprivacy** | Option<[**Vec<String>**](String.md)> | Must have this geoprivacy setting |  |
**taxon_geoprivacy** | Option<[**Vec<String>**](String.md)> | Filter observations by the most conservative geoprivacy applied by a conservation status associated with one of the taxa proposed in the current identifications.  |  |
**hrank** | Option<**String**> | Taxon must have this rank or lower |  |
**lrank** | Option<**String**> | Taxon must have this rank or higher |  |
**iconic_taxa** | Option<[**Vec<String>**](String.md)> | Taxon must by within this iconic taxon |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**identifications** | Option<**String**> | Identifications must meet these criteria |  |
**lat** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**lng** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**radius** | Option<**String**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**nelat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**nelng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlat** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**swlng** | Option<**f64**> | Must be within this bounding box (*nelat, *nelng, *swlat, *swlng)  |  |
**list_id** | Option<**i32**> | Taxon must be in the list with this ID |  |
**not_in_project** | Option<**String**> | Must not be in the project with this ID or slug |  |
**not_matching_project_rules_for** | Option<**String**> | Must not match the rules of the project with this ID or slug |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |
**locale** | Option<**String**> | Locale preference for taxon common names  |  |
**preferred_place_id** | Option<**i32**> | Place preference for regional taxon common names  |  |
**ttl** | Option<**String**> | Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers  |  |

### Return type

[**crate::models::SpeciesCountsResponse**](SpeciesCountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observations_updates_get

> observations_updates_get(created_after, viewed, observations_by, page, per_page)
Observation User Updates

Given zero to many of following parameters, returns an array of objects representing new comments and identifications on observations the authenticated user has subscribed to. Requires authentication 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_after** | Option<**String**> | Must be created at or after this time |  |
**viewed** | Option<**bool**> | Notification has been viewed by the user before |  |
**observations_by** | Option<**String**> | Only show updates on observations owned by the currently authenticated user or on observations the authenticated user is following but does not own.  |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscriptions_observation_id_subscribe_post

> subscriptions_observation_id_subscribe_post(id)
Observation Subscribe

Toggles current user's subscription to this observation. If the logged-in user is not subscribed, POSTing here will subscribe them. If they are already subscribed, this will remove the subscription 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_unvote_observation_id_delete

> votes_unvote_observation_id_delete(id, body)
Observation Unvote

Remove a vote from an observation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PostObservationVote**](PostObservationVote.md)> | Vote object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_vote_observation_id_post

> votes_vote_observation_id_post(id, body)
Observation Vote

Vote on an observation. A vote with an empty `scope` is recorded as a `fave` of the observation. A vote with scope `needs_id` is recorded as a vote on the Quality Grade criterion \"can the Community ID still be confirmed or improved?\", and can be an up or down vote 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PostObservationVote**](PostObservationVote.md)> | Vote object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

