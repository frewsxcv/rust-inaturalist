# \PlacesApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**places_autocomplete_get**](PlacesApi.md#places_autocomplete_get) | **GET** /places/autocomplete | Place Autocomplete
[**places_id_get**](PlacesApi.md#places_id_get) | **GET** /places/{id} | Place Details
[**places_nearby_get**](PlacesApi.md#places_nearby_get) | **GET** /places/nearby | Nearby Places



## places_autocomplete_get

> crate::models::PlacesResponse places_autocomplete_get(q, order_by)
Place Autocomplete

Given an string, returns places with names starting with the search term. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Name must begin with this value | [required] |
**order_by** | Option<**String**> | Sort field |  |

### Return type

[**crate::models::PlacesResponse**](PlacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## places_id_get

> crate::models::PlacesResponse places_id_get(id, admin_level)
Place Details

Given an ID, or an array of IDs in comma-delimited format, returns corresponding places. A maximum of 500 results will be returned 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<String>**](String.md) | Must have this ID or slug | [required] |
**admin_level** | Option<[**Vec<i32>**](i32.md)> | Admin level of a place, or an array of admin levels in comma-delimited format. Supported admin levels are: -10 (continent), 0 (country), 10 (state), 20 (county), 30 (town), 100 (park) |  |

### Return type

[**crate::models::PlacesResponse**](PlacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## places_nearby_get

> crate::models::NearbyPlacesResponse places_nearby_get(nelat, nelng, swlat, swlng, name, per_page)
Nearby Places

Given an bounding box, and an optional name query, return `standard` iNaturalist curator approved and `community` non-curated places nearby 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nelat** | **f64** | Must be nearby this bounding box (*nelat, *nelng, *swlat, *swlng)  | [required] |
**nelng** | **f64** | Must be nearby this bounding box (*nelat, *nelng, *swlat, *swlng)  | [required] |
**swlat** | **f64** | Must be nearby this bounding box (*nelat, *nelng, *swlat, *swlng)  | [required] |
**swlng** | **f64** | Must be nearby this bounding box (*nelat, *nelng, *swlat, *swlng)  | [required] |
**name** | Option<**String**> | Name must match this value |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |

### Return type

[**crate::models::NearbyPlacesResponse**](NearbyPlacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

