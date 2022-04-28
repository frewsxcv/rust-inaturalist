# \IdentificationsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**identifications_categories_get**](IdentificationsApi.md#identifications_categories_get) | **GET** /identifications/categories | Identification Categories
[**identifications_get**](IdentificationsApi.md#identifications_get) | **GET** /identifications | Identification Search
[**identifications_id_delete**](IdentificationsApi.md#identifications_id_delete) | **DELETE** /identifications/{id} | Identification Delete
[**identifications_id_get**](IdentificationsApi.md#identifications_id_get) | **GET** /identifications/{id} | Identification Details
[**identifications_id_put**](IdentificationsApi.md#identifications_id_put) | **PUT** /identifications/{id} | Identification Update
[**identifications_identifiers_get**](IdentificationsApi.md#identifications_identifiers_get) | **GET** /identifications/identifiers | Identification Identifiers
[**identifications_observers_get**](IdentificationsApi.md#identifications_observers_get) | **GET** /identifications/observers | Identification Observers
[**identifications_post**](IdentificationsApi.md#identifications_post) | **POST** /identifications | Identification Create
[**identifications_recent_taxa_get**](IdentificationsApi.md#identifications_recent_taxa_get) | **GET** /identifications/recent_taxa | Identification Recent Taxa
[**identifications_similar_species_get**](IdentificationsApi.md#identifications_similar_species_get) | **GET** /identifications/similar_species | Identification Similar Species
[**identifications_species_counts_get**](IdentificationsApi.md#identifications_species_counts_get) | **GET** /identifications/species_counts | Identification Species Counts



## identifications_categories_get

> identifications_categories_get(current_taxon, own_observation, is_change, taxon_active, observation_taxon_active, id, rank, observation_rank, user_id, user_login, current, category, place_id, quality_grade, taxon_id, observation_taxon_id, iconic_taxon_id, observation_iconic_taxon_id, lrank, hrank, observation_lrank, observation_hrank, without_taxon_id, without_observation_taxon_id, d1, d2, observation_created_d1, observation_created_d2, observed_d1, observed_d2, id_above, id_below, page, per_page, order, order_by, only_id)
Identification Categories

Given zero to many of following parameters, return counts of the categories of identifications matching the search criteria 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**current_taxon** | Option<**bool**> | ID's taxon is the same it's observation's taxon |  |
**own_observation** | Option<**bool**> | ID was added by the observer |  |
**is_change** | Option<**bool**> | ID was created as a results of a taxon change |  |
**taxon_active** | Option<**bool**> | ID's taxon is currently an active taxon |  |
**observation_taxon_active** | Option<**bool**> | Observation's taxon is currently an active taxon |  |
**id** | Option<[**Vec<i32>**](i32.md)> | Identification ID |  |
**rank** | Option<[**Vec<String>**](String.md)> | ID's taxon must have this rank |  |
**observation_rank** | Option<[**Vec<String>**](String.md)> | Observation's taxon must have this rank |  |
**user_id** | Option<[**Vec<i32>**](i32.md)> | Identifier must have this user ID |  |
**user_login** | Option<[**Vec<String>**](String.md)> | Identifier must have this login |  |
**current** | Option<**bool**> | Most recent ID on a observation by a user |  |[default to true]
**category** | Option<[**Vec<String>**](String.md)> | Type of identification |  |
**place_id** | Option<[**Vec<String>**](String.md)> | Observation must occur in this place |  |
**quality_grade** | Option<[**Vec<String>**](String.md)> | Observation must have this quality grade |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | ID taxa must match the given taxa or their descendants |  |
**observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation taxa must match the given taxa or their descendants |  |
**iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | ID iconic taxon ID |  |
**observation_iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation iconic taxon ID |  |
**lrank** | Option<**String**> | ID taxon must have this rank or higher |  |
**hrank** | Option<**String**> | ID taxon must have this rank or lower |  |
**observation_lrank** | Option<**String**> | Observation taxon must have this rank or higher |  |
**observation_hrank** | Option<**String**> | Observation taxon must have this rank or lower |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of these taxa and their descendants |  |
**without_observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of observations of these taxa and their descendants |  |
**d1** | Option<**String**> | ID created on or after this time |  |
**d2** | Option<**String**> | ID created on or before this time |  |
**observation_created_d1** | Option<**String**> | Observation created on or after this date |  |
**observation_created_d2** | Option<**String**> | Observation created on or before this date |  |
**observed_d1** | Option<**String**> | Observation observed on or after this date |  |
**observed_d2** | Option<**String**> | Observation observed on or before this date |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**order** | Option<**String**> | Sort order |  |[default to desc]
**order_by** | Option<**String**> | Sort field |  |[default to created_at]
**only_id** | Option<**bool**> | Return only the record IDs |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identifications_get

> identifications_get(current_taxon, own_observation, is_change, taxon_active, observation_taxon_active, id, rank, observation_rank, user_id, user_login, current, category, place_id, quality_grade, taxon_id, observation_taxon_id, iconic_taxon_id, observation_iconic_taxon_id, lrank, hrank, observation_lrank, observation_hrank, without_taxon_id, without_observation_taxon_id, d1, d2, observation_created_d1, observation_created_d2, observed_d1, observed_d2, id_above, id_below, page, per_page, order, order_by, only_id)
Identification Search

Given zero to many of following parameters, returns identifications matching the search criteria 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**current_taxon** | Option<**bool**> | ID's taxon is the same it's observation's taxon |  |
**own_observation** | Option<**bool**> | ID was added by the observer |  |
**is_change** | Option<**bool**> | ID was created as a results of a taxon change |  |
**taxon_active** | Option<**bool**> | ID's taxon is currently an active taxon |  |
**observation_taxon_active** | Option<**bool**> | Observation's taxon is currently an active taxon |  |
**id** | Option<[**Vec<i32>**](i32.md)> | Identification ID |  |
**rank** | Option<[**Vec<String>**](String.md)> | ID's taxon must have this rank |  |
**observation_rank** | Option<[**Vec<String>**](String.md)> | Observation's taxon must have this rank |  |
**user_id** | Option<[**Vec<i32>**](i32.md)> | Identifier must have this user ID |  |
**user_login** | Option<[**Vec<String>**](String.md)> | Identifier must have this login |  |
**current** | Option<**bool**> | Most recent ID on a observation by a user |  |[default to true]
**category** | Option<[**Vec<String>**](String.md)> | Type of identification |  |
**place_id** | Option<[**Vec<String>**](String.md)> | Observation must occur in this place |  |
**quality_grade** | Option<[**Vec<String>**](String.md)> | Observation must have this quality grade |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | ID taxa must match the given taxa or their descendants |  |
**observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation taxa must match the given taxa or their descendants |  |
**iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | ID iconic taxon ID |  |
**observation_iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation iconic taxon ID |  |
**lrank** | Option<**String**> | ID taxon must have this rank or higher |  |
**hrank** | Option<**String**> | ID taxon must have this rank or lower |  |
**observation_lrank** | Option<**String**> | Observation taxon must have this rank or higher |  |
**observation_hrank** | Option<**String**> | Observation taxon must have this rank or lower |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of these taxa and their descendants |  |
**without_observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of observations of these taxa and their descendants |  |
**d1** | Option<**String**> | ID created on or after this time |  |
**d2** | Option<**String**> | ID created on or before this time |  |
**observation_created_d1** | Option<**String**> | Observation created on or after this date |  |
**observation_created_d2** | Option<**String**> | Observation created on or before this date |  |
**observed_d1** | Option<**String**> | Observation observed on or after this date |  |
**observed_d2** | Option<**String**> | Observation observed on or before this date |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**order** | Option<**String**> | Sort order |  |[default to desc]
**order_by** | Option<**String**> | Sort field |  |[default to created_at]
**only_id** | Option<**bool**> | Return only the record IDs |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identifications_id_delete

> identifications_id_delete(id)
Identification Delete

Delete an identification. See description of `PUT /identifications/{id} for notes on withdrawing and restoring identifications. 

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


## identifications_id_get

> identifications_id_get(id)
Identification Details

Given an ID, or an array of IDs in comma-delimited format, returns corresponding identifications. A maximum of 30 results will be returned 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<i32>**](i32.md) | Must have this ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identifications_id_put

> identifications_id_put(id, body)
Identification Update

Update an identification. Note that to \"withdraw\" an observation you send a `PUT` request to this endpoint and set the `current` attribute to false. To \"restore\" it you do the same but set `current` to `true`. Only one identification by a given user can be `current` for a given observation, so if you \"restore\" one all the other identifications by the authenticated user for the given observation will be withdrawn. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PostIdentification**](PostIdentification.md)> | Identification object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identifications_identifiers_get

> crate::models::UserCountsResponse identifications_identifiers_get(current_taxon, own_observation, is_change, taxon_active, observation_taxon_active, id, rank, observation_rank, user_id, user_login, current, category, place_id, quality_grade, taxon_id, observation_taxon_id, iconic_taxon_id, observation_iconic_taxon_id, lrank, hrank, observation_lrank, observation_hrank, without_taxon_id, without_observation_taxon_id, d1, d2, observation_created_d1, observation_created_d2, observed_d1, observed_d2, id_above, id_below, page, per_page, order, order_by, only_id)
Identification Identifiers

Given zero to many of following parameters, returns creators of identifications matching the search criteria and the count of matching identifications, ordered by count descending 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**current_taxon** | Option<**bool**> | ID's taxon is the same it's observation's taxon |  |
**own_observation** | Option<**bool**> | ID was added by the observer |  |
**is_change** | Option<**bool**> | ID was created as a results of a taxon change |  |
**taxon_active** | Option<**bool**> | ID's taxon is currently an active taxon |  |
**observation_taxon_active** | Option<**bool**> | Observation's taxon is currently an active taxon |  |
**id** | Option<[**Vec<i32>**](i32.md)> | Identification ID |  |
**rank** | Option<[**Vec<String>**](String.md)> | ID's taxon must have this rank |  |
**observation_rank** | Option<[**Vec<String>**](String.md)> | Observation's taxon must have this rank |  |
**user_id** | Option<[**Vec<i32>**](i32.md)> | Identifier must have this user ID |  |
**user_login** | Option<[**Vec<String>**](String.md)> | Identifier must have this login |  |
**current** | Option<**bool**> | Most recent ID on a observation by a user |  |[default to true]
**category** | Option<[**Vec<String>**](String.md)> | Type of identification |  |
**place_id** | Option<[**Vec<String>**](String.md)> | Observation must occur in this place |  |
**quality_grade** | Option<[**Vec<String>**](String.md)> | Observation must have this quality grade |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | ID taxa must match the given taxa or their descendants |  |
**observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation taxa must match the given taxa or their descendants |  |
**iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | ID iconic taxon ID |  |
**observation_iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation iconic taxon ID |  |
**lrank** | Option<**String**> | ID taxon must have this rank or higher |  |
**hrank** | Option<**String**> | ID taxon must have this rank or lower |  |
**observation_lrank** | Option<**String**> | Observation taxon must have this rank or higher |  |
**observation_hrank** | Option<**String**> | Observation taxon must have this rank or lower |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of these taxa and their descendants |  |
**without_observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of observations of these taxa and their descendants |  |
**d1** | Option<**String**> | ID created on or after this time |  |
**d2** | Option<**String**> | ID created on or before this time |  |
**observation_created_d1** | Option<**String**> | Observation created on or after this date |  |
**observation_created_d2** | Option<**String**> | Observation created on or before this date |  |
**observed_d1** | Option<**String**> | Observation observed on or after this date |  |
**observed_d2** | Option<**String**> | Observation observed on or before this date |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**order** | Option<**String**> | Sort order |  |[default to desc]
**order_by** | Option<**String**> | Sort field |  |[default to created_at]
**only_id** | Option<**bool**> | Return only the record IDs |  |

### Return type

[**crate::models::UserCountsResponse**](UserCountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identifications_observers_get

> crate::models::UserCountsResponse identifications_observers_get(current_taxon, own_observation, is_change, taxon_active, observation_taxon_active, id, rank, observation_rank, user_id, user_login, current, category, place_id, quality_grade, taxon_id, observation_taxon_id, iconic_taxon_id, observation_iconic_taxon_id, lrank, hrank, observation_lrank, observation_hrank, without_taxon_id, without_observation_taxon_id, d1, d2, observation_created_d1, observation_created_d2, observed_d1, observed_d2, id_above, id_below, page, per_page, order, order_by, only_id)
Identification Observers

Given zero to many of following parameters, returns creators of observations of identifications matching the search criteria and the count of matching observations, ordered by count descending 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**current_taxon** | Option<**bool**> | ID's taxon is the same it's observation's taxon |  |
**own_observation** | Option<**bool**> | ID was added by the observer |  |
**is_change** | Option<**bool**> | ID was created as a results of a taxon change |  |
**taxon_active** | Option<**bool**> | ID's taxon is currently an active taxon |  |
**observation_taxon_active** | Option<**bool**> | Observation's taxon is currently an active taxon |  |
**id** | Option<[**Vec<i32>**](i32.md)> | Identification ID |  |
**rank** | Option<[**Vec<String>**](String.md)> | ID's taxon must have this rank |  |
**observation_rank** | Option<[**Vec<String>**](String.md)> | Observation's taxon must have this rank |  |
**user_id** | Option<[**Vec<i32>**](i32.md)> | Identifier must have this user ID |  |
**user_login** | Option<[**Vec<String>**](String.md)> | Identifier must have this login |  |
**current** | Option<**bool**> | Most recent ID on a observation by a user |  |[default to true]
**category** | Option<[**Vec<String>**](String.md)> | Type of identification |  |
**place_id** | Option<[**Vec<String>**](String.md)> | Observation must occur in this place |  |
**quality_grade** | Option<[**Vec<String>**](String.md)> | Observation must have this quality grade |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | ID taxa must match the given taxa or their descendants |  |
**observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation taxa must match the given taxa or their descendants |  |
**iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | ID iconic taxon ID |  |
**observation_iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation iconic taxon ID |  |
**lrank** | Option<**String**> | ID taxon must have this rank or higher |  |
**hrank** | Option<**String**> | ID taxon must have this rank or lower |  |
**observation_lrank** | Option<**String**> | Observation taxon must have this rank or higher |  |
**observation_hrank** | Option<**String**> | Observation taxon must have this rank or lower |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of these taxa and their descendants |  |
**without_observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of observations of these taxa and their descendants |  |
**d1** | Option<**String**> | ID created on or after this time |  |
**d2** | Option<**String**> | ID created on or before this time |  |
**observation_created_d1** | Option<**String**> | Observation created on or after this date |  |
**observation_created_d2** | Option<**String**> | Observation created on or before this date |  |
**observed_d1** | Option<**String**> | Observation observed on or after this date |  |
**observed_d2** | Option<**String**> | Observation observed on or before this date |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**order** | Option<**String**> | Sort order |  |[default to desc]
**order_by** | Option<**String**> | Sort field |  |[default to created_at]
**only_id** | Option<**bool**> | Return only the record IDs |  |

### Return type

[**crate::models::UserCountsResponse**](UserCountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identifications_post

> identifications_post(body)
Identification Create

Create an identification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostIdentification**](PostIdentification.md)> | Identification object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identifications_recent_taxa_get

> identifications_recent_taxa_get(current_taxon, own_observation, is_change, taxon_active, observation_taxon_active, id, rank, observation_rank, user_id, user_login, current, category, place_id, quality_grade, taxon_id, observation_taxon_id, iconic_taxon_id, observation_iconic_taxon_id, lrank, hrank, observation_lrank, observation_hrank, without_taxon_id, without_observation_taxon_id, d1, d2, observation_created_d1, observation_created_d2, observed_d1, observed_d2, id_above, id_below, page, per_page, order, order_by, only_id)
Identification Recent Taxa

Returns an array of objects each containing an identification and a taxon. Returns IDs representing the earliest occurrence of taxa associated with identifications in the filtered set of results 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**current_taxon** | Option<**bool**> | ID's taxon is the same it's observation's taxon |  |
**own_observation** | Option<**bool**> | ID was added by the observer |  |
**is_change** | Option<**bool**> | ID was created as a results of a taxon change |  |
**taxon_active** | Option<**bool**> | ID's taxon is currently an active taxon |  |
**observation_taxon_active** | Option<**bool**> | Observation's taxon is currently an active taxon |  |
**id** | Option<[**Vec<i32>**](i32.md)> | Identification ID |  |
**rank** | Option<[**Vec<String>**](String.md)> | ID's taxon must have this rank |  |
**observation_rank** | Option<[**Vec<String>**](String.md)> | Observation's taxon must have this rank |  |
**user_id** | Option<[**Vec<i32>**](i32.md)> | Identifier must have this user ID |  |
**user_login** | Option<[**Vec<String>**](String.md)> | Identifier must have this login |  |
**current** | Option<**bool**> | Most recent ID on a observation by a user |  |[default to true]
**category** | Option<[**Vec<String>**](String.md)> | Type of identification |  |
**place_id** | Option<[**Vec<String>**](String.md)> | Observation must occur in this place |  |
**quality_grade** | Option<[**Vec<String>**](String.md)> | Observation must have this quality grade |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | ID taxa must match the given taxa or their descendants |  |
**observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation taxa must match the given taxa or their descendants |  |
**iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | ID iconic taxon ID |  |
**observation_iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation iconic taxon ID |  |
**lrank** | Option<**String**> | ID taxon must have this rank or higher |  |
**hrank** | Option<**String**> | ID taxon must have this rank or lower |  |
**observation_lrank** | Option<**String**> | Observation taxon must have this rank or higher |  |
**observation_hrank** | Option<**String**> | Observation taxon must have this rank or lower |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of these taxa and their descendants |  |
**without_observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of observations of these taxa and their descendants |  |
**d1** | Option<**String**> | ID created on or after this time |  |
**d2** | Option<**String**> | ID created on or before this time |  |
**observation_created_d1** | Option<**String**> | Observation created on or after this date |  |
**observation_created_d2** | Option<**String**> | Observation created on or before this date |  |
**observed_d1** | Option<**String**> | Observation observed on or after this date |  |
**observed_d2** | Option<**String**> | Observation observed on or before this date |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**order** | Option<**String**> | Sort order |  |[default to desc]
**order_by** | Option<**String**> | Sort field |  |[default to created_at]
**only_id** | Option<**bool**> | Return only the record IDs |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identifications_similar_species_get

> identifications_similar_species_get(taxon_id, acc, captive, endemic, geo, id_please, identified, introduced, mappable, native, out_of_range, pcid, photos, popular, sounds, taxon_is_active, threatened, verifiable, licensed, photo_licensed, id, not_id, license, ofv_datatype, photo_license, place_id, project_id, rank, site_id, sound_license, without_taxon_id, taxon_name, user_id, user_login, ident_user_id, day, month, year, term_id, term_value_id, without_term_value_id, acc_above, acc_below, acc_below_or_unknown, d1, d2, created_d1, created_d2, created_on, observed_on, unobserved_by_user_id, apply_project_rules_for, cs, csa, csi, geoprivacy, taxon_geoprivacy, hrank, lrank, iconic_taxa, id_above, id_below, identifications, lat, lng, radius, nelat, nelng, swlat, swlng, list_id, not_in_project, not_matching_project_rules_for, q, search_on, quality_grade, updated_since, viewer_id, reviewed)
Identification Similar Species

Returns species attached to IDs of observations of this taxon, or attached to observations identified as this species, ordered by combined frequency descending. This will only return species in the same iconic taxon, and will never return descendants of the chosen taxon 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**taxon_id** | **i32** | Only show observations of these taxa and their descendants | [required] |
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

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## identifications_species_counts_get

> crate::models::SpeciesCountsResponse identifications_species_counts_get(current_taxon, own_observation, is_change, taxon_active, observation_taxon_active, id, rank, observation_rank, user_id, user_login, current, category, place_id, quality_grade, taxon_id, observation_taxon_id, iconic_taxon_id, observation_iconic_taxon_id, lrank, hrank, observation_lrank, observation_hrank, without_taxon_id, without_observation_taxon_id, d1, d2, observation_created_d1, observation_created_d2, observed_d1, observed_d2, id_above, id_below, page, per_page, order, order_by, only_id, taxon_of)
Identification Species Counts

Given zero to many of following parameters, returns `leaf taxa` associated with identifications matching the search criteria and the count of identifications they are associated with, ordered by count descending. `Leaf taxa` are the leaves of the taxonomic tree containing only the taxa associated with observations matching the search criteria. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**current_taxon** | Option<**bool**> | ID's taxon is the same it's observation's taxon |  |
**own_observation** | Option<**bool**> | ID was added by the observer |  |
**is_change** | Option<**bool**> | ID was created as a results of a taxon change |  |
**taxon_active** | Option<**bool**> | ID's taxon is currently an active taxon |  |
**observation_taxon_active** | Option<**bool**> | Observation's taxon is currently an active taxon |  |
**id** | Option<[**Vec<i32>**](i32.md)> | Identification ID |  |
**rank** | Option<[**Vec<String>**](String.md)> | ID's taxon must have this rank |  |
**observation_rank** | Option<[**Vec<String>**](String.md)> | Observation's taxon must have this rank |  |
**user_id** | Option<[**Vec<i32>**](i32.md)> | Identifier must have this user ID |  |
**user_login** | Option<[**Vec<String>**](String.md)> | Identifier must have this login |  |
**current** | Option<**bool**> | Most recent ID on a observation by a user |  |[default to true]
**category** | Option<[**Vec<String>**](String.md)> | Type of identification |  |
**place_id** | Option<[**Vec<String>**](String.md)> | Observation must occur in this place |  |
**quality_grade** | Option<[**Vec<String>**](String.md)> | Observation must have this quality grade |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | ID taxa must match the given taxa or their descendants |  |
**observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation taxa must match the given taxa or their descendants |  |
**iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | ID iconic taxon ID |  |
**observation_iconic_taxon_id** | Option<[**Vec<String>**](String.md)> | Observation iconic taxon ID |  |
**lrank** | Option<**String**> | ID taxon must have this rank or higher |  |
**hrank** | Option<**String**> | ID taxon must have this rank or lower |  |
**observation_lrank** | Option<**String**> | Observation taxon must have this rank or higher |  |
**observation_hrank** | Option<**String**> | Observation taxon must have this rank or lower |  |
**without_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of these taxa and their descendants |  |
**without_observation_taxon_id** | Option<[**Vec<String>**](String.md)> | Exclude IDs of observations of these taxa and their descendants |  |
**d1** | Option<**String**> | ID created on or after this time |  |
**d2** | Option<**String**> | ID created on or before this time |  |
**observation_created_d1** | Option<**String**> | Observation created on or after this date |  |
**observation_created_d2** | Option<**String**> | Observation created on or before this date |  |
**observed_d1** | Option<**String**> | Observation observed on or after this date |  |
**observed_d2** | Option<**String**> | Observation observed on or before this date |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**order** | Option<**String**> | Sort order |  |[default to desc]
**order_by** | Option<**String**> | Sort field |  |[default to created_at]
**only_id** | Option<**bool**> | Return only the record IDs |  |
**taxon_of** | Option<**String**> | Source of the taxon for counting |  |[default to identification]

### Return type

[**crate::models::SpeciesCountsResponse**](SpeciesCountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

