# \ObservationTilesApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**colored_heatmap_zoom_xy_png_get**](ObservationTilesApi.md#colored_heatmap_zoom_xy_png_get) | **GET** /colored_heatmap/{zoom}/{x}/{y}.png | Colored Heatmap Tiles
[**grid_zoom_xy_png_get**](ObservationTilesApi.md#grid_zoom_xy_png_get) | **GET** /grid/{zoom}/{x}/{y}.png | Grid Tiles
[**heatmap_zoom_xy_png_get**](ObservationTilesApi.md#heatmap_zoom_xy_png_get) | **GET** /heatmap/{zoom}/{x}/{y}.png | Heatmap Tiles
[**points_zoom_xy_png_get**](ObservationTilesApi.md#points_zoom_xy_png_get) | **GET** /points/{zoom}/{x}/{y}.png | Points Tiles



## colored_heatmap_zoom_xy_png_get

> colored_heatmap_zoom_xy_png_get(zoom, x, y, color, acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, expected_nearby, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, hour, day, month, year, created_day, created_month, created_year, term_id, term_value_id, without_term_id, without_term_value_id, term_id_or_unknown, annotation_user_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, obscuration, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, observation_accuracy_experiment_id, q, search_on, quality_grade, updated_since, viewer_id, reviewed)
Colored Heatmap Tiles

Given zero to many of following parameters, returns a PNG image representing the matching observations within a map tile, following the XYZ tiling scheme 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zoom** | **i32** | Zoom level. Z coordinate in the XYZ tiling scheme | [required] |
**x** | **i32** | X coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**y** | **i32** | Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**color** | Option<**String**> | Primary color to use in tile creation. Accepts common colors by string (e.g. `color=blue`), and accepts escaped color HEX codes (e.g. `color=%2386a91c`)  |  |
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
**expected_nearby** | Option<**bool**> | Observation taxon is expected nearby |  |
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
**hour** | Option<[**Vec<String>**](String.md)> | Must be observed within this hour of the day |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**created_day** | Option<[**Vec<String>**](String.md)> | Must be created within this day of the month |  |
**created_month** | Option<[**Vec<String>**](String.md)> | Must be created within this month |  |
**created_year** | Option<[**Vec<String>**](String.md)> | Must be created within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**term_id_or_unknown** | Option<[**Vec<i32>**](i32.md)> | Must be combined with the `term_value_id` or the `without_term_value_id` parameter. Must have an annotation using this controlled term ID and associated term value IDs or be missing this annotation.  |  |
**annotation_user_id** | Option<[**Vec<String>**](String.md)> | Must have an annotation created by this user  |  |
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
**obscuration** | Option<[**Vec<String>**](String.md)> | Must have `geoprivacy` or `taxon_geoprivacy` fields matching these values  |  |
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
**observation_accuracy_experiment_id** | Option<[**Vec<i32>**](i32.md)> | Must included in this observation accuracy experiment |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grid_zoom_xy_png_get

> grid_zoom_xy_png_get(zoom, x, y, color, acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, expected_nearby, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, hour, day, month, year, created_day, created_month, created_year, term_id, term_value_id, without_term_id, without_term_value_id, term_id_or_unknown, annotation_user_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, obscuration, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, observation_accuracy_experiment_id, q, search_on, quality_grade, updated_since, viewer_id, reviewed)
Grid Tiles

Given zero to many of following parameters, returns a PNG image representing the matching observations within a map tile, following the XYZ tiling scheme 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zoom** | **i32** | Zoom level. Z coordinate in the XYZ tiling scheme | [required] |
**x** | **i32** | X coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**y** | **i32** | Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**color** | Option<**String**> | Primary color to use in tile creation. Accepts common colors by string (e.g. `color=blue`), and accepts escaped color HEX codes (e.g. `color=%2386a91c`)  |  |
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
**expected_nearby** | Option<**bool**> | Observation taxon is expected nearby |  |
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
**hour** | Option<[**Vec<String>**](String.md)> | Must be observed within this hour of the day |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**created_day** | Option<[**Vec<String>**](String.md)> | Must be created within this day of the month |  |
**created_month** | Option<[**Vec<String>**](String.md)> | Must be created within this month |  |
**created_year** | Option<[**Vec<String>**](String.md)> | Must be created within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**term_id_or_unknown** | Option<[**Vec<i32>**](i32.md)> | Must be combined with the `term_value_id` or the `without_term_value_id` parameter. Must have an annotation using this controlled term ID and associated term value IDs or be missing this annotation.  |  |
**annotation_user_id** | Option<[**Vec<String>**](String.md)> | Must have an annotation created by this user  |  |
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
**obscuration** | Option<[**Vec<String>**](String.md)> | Must have `geoprivacy` or `taxon_geoprivacy` fields matching these values  |  |
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
**observation_accuracy_experiment_id** | Option<[**Vec<i32>**](i32.md)> | Must included in this observation accuracy experiment |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## heatmap_zoom_xy_png_get

> heatmap_zoom_xy_png_get(zoom, x, y, color, acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, expected_nearby, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, hour, day, month, year, created_day, created_month, created_year, term_id, term_value_id, without_term_id, without_term_value_id, term_id_or_unknown, annotation_user_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, obscuration, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, observation_accuracy_experiment_id, q, search_on, quality_grade, updated_since, viewer_id, reviewed)
Heatmap Tiles

Given zero to many of following parameters, returns a PNG image representing the matching observations within a map tile, following the XYZ tiling scheme 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zoom** | **i32** | Zoom level. Z coordinate in the XYZ tiling scheme | [required] |
**x** | **i32** | X coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**y** | **i32** | Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**color** | Option<**String**> | Primary color to use in tile creation. Accepts common colors by string (e.g. `color=blue`), and accepts escaped color HEX codes (e.g. `color=%2386a91c`)  |  |
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
**expected_nearby** | Option<**bool**> | Observation taxon is expected nearby |  |
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
**hour** | Option<[**Vec<String>**](String.md)> | Must be observed within this hour of the day |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**created_day** | Option<[**Vec<String>**](String.md)> | Must be created within this day of the month |  |
**created_month** | Option<[**Vec<String>**](String.md)> | Must be created within this month |  |
**created_year** | Option<[**Vec<String>**](String.md)> | Must be created within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**term_id_or_unknown** | Option<[**Vec<i32>**](i32.md)> | Must be combined with the `term_value_id` or the `without_term_value_id` parameter. Must have an annotation using this controlled term ID and associated term value IDs or be missing this annotation.  |  |
**annotation_user_id** | Option<[**Vec<String>**](String.md)> | Must have an annotation created by this user  |  |
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
**obscuration** | Option<[**Vec<String>**](String.md)> | Must have `geoprivacy` or `taxon_geoprivacy` fields matching these values  |  |
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
**observation_accuracy_experiment_id** | Option<[**Vec<i32>**](i32.md)> | Must included in this observation accuracy experiment |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## points_zoom_xy_png_get

> points_zoom_xy_png_get(zoom, x, y, color, acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, expected_nearby, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, taxon_id, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, hour, day, month, year, created_day, created_month, created_year, term_id, term_value_id, without_term_id, without_term_value_id, term_id_or_unknown, annotation_user_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, obscuration, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, observation_accuracy_experiment_id, q, search_on, quality_grade, updated_since, viewer_id, reviewed)
Points Tiles

Given zero to many of following parameters, returns a PNG image representing the matching observations within a map tile, following the XYZ tiling scheme 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zoom** | **i32** | Zoom level. Z coordinate in the XYZ tiling scheme | [required] |
**x** | **i32** | X coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**y** | **i32** | Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**color** | Option<**String**> | Primary color to use in tile creation. Accepts common colors by string (e.g. `color=blue`), and accepts escaped color HEX codes (e.g. `color=%2386a91c`)  |  |
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
**expected_nearby** | Option<**bool**> | Observation taxon is expected nearby |  |
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
**hour** | Option<[**Vec<String>**](String.md)> | Must be observed within this hour of the day |  |
**day** | Option<[**Vec<String>**](String.md)> | Must be observed within this day of the month |  |
**month** | Option<[**Vec<String>**](String.md)> | Must be observed within this month |  |
**year** | Option<[**Vec<String>**](String.md)> | Must be observed within this year |  |
**created_day** | Option<[**Vec<String>**](String.md)> | Must be created within this day of the month |  |
**created_month** | Option<[**Vec<String>**](String.md)> | Must be created within this month |  |
**created_year** | Option<[**Vec<String>**](String.md)> | Must be created within this year |  |
**term_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled term ID |  |
**term_value_id** | Option<[**Vec<i32>**](i32.md)> | Must have an annotation using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**without_term_id** | Option<**i32**> | Exclude observations with annotations using this controlled value ID.  |  |
**without_term_value_id** | Option<[**Vec<i32>**](i32.md)> | Exclude observations with annotations using this controlled value ID. Must be combined with the `term_id` parameter  |  |
**term_id_or_unknown** | Option<[**Vec<i32>**](i32.md)> | Must be combined with the `term_value_id` or the `without_term_value_id` parameter. Must have an annotation using this controlled term ID and associated term value IDs or be missing this annotation.  |  |
**annotation_user_id** | Option<[**Vec<String>**](String.md)> | Must have an annotation created by this user  |  |
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
**obscuration** | Option<[**Vec<String>**](String.md)> | Must have `geoprivacy` or `taxon_geoprivacy` fields matching these values  |  |
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
**observation_accuracy_experiment_id** | Option<[**Vec<i32>**](i32.md)> | Must included in this observation accuracy experiment |  |
**q** | Option<**String**> | Search observation properties. Can be combined with `search_on` |  |
**search_on** | Option<**String**> | Properties to search on, when combined with `q`. Searches across all properties by default  |  |
**quality_grade** | Option<**String**> | Must have this quality grade |  |
**updated_since** | Option<**String**> | Must be updated since this time |  |
**viewer_id** | Option<**String**> | See `reviewed` |  |
**reviewed** | Option<**bool**> | Observations have been reviewed by the user with ID equal to the value of the `viewer_id` parameter  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

