# \SearchApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_get**](SearchApi.md#search_get) | **GET** /search | Site Search



## search_get

> search_get(q, sources, place_id, per_page, locale, preferred_place_id)
Site Search

Given zero to many of following parameters, returns object matching the search criteria 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Search object properties |  |
**sources** | Option<[**Vec<String>**](String.md)> | Must be of this type |  |
**place_id** | Option<[**Vec<String>**](String.md)> | Must be associated with this place |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**locale** | Option<**String**> | Locale preference for taxon common names  |  |
**preferred_place_id** | Option<**i32**> | Place preference for regional taxon common names  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

