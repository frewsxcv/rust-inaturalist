# \TaxaApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**taxa_autocomplete_get**](TaxaApi.md#taxa_autocomplete_get) | **GET** /taxa/autocomplete | Taxon Autocomplete
[**taxa_get**](TaxaApi.md#taxa_get) | **GET** /taxa | Taxon Search
[**taxa_id_get**](TaxaApi.md#taxa_id_get) | **GET** /taxa/{id} | Taxon Details



## taxa_autocomplete_get

> crate::models::TaxaAutocompleteResponse taxa_autocomplete_get(q, is_active, taxon_id, rank, rank_level, per_page, locale, preferred_place_id, all_names)
Taxon Autocomplete

Given an string, returns taxa with names starting with the search term 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Name must begin with this value | [required] |
**is_active** | Option<**bool**> | Taxon is `active` |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | Only show taxa with this ID, or its descendants |  |
**rank** | Option<[**Vec<String>**](String.md)> | Taxon must have this rank |  |
**rank_level** | Option<**f32**> | Taxon must have this rank level. Some example values are 70 (kingdom), 60 (phylum), 50 (class), 40 (order), 30 (family), 20 (genus), 10 (species), 5 (subspecies)  |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is 30 for this endpoint |  |
**locale** | Option<**String**> | Locale preference for taxon common names  |  |
**preferred_place_id** | Option<**i32**> | Place preference for regional taxon common names  |  |
**all_names** | Option<**bool**> | Include all taxon names in the response |  |

### Return type

[**crate::models::TaxaAutocompleteResponse**](TaxaAutocompleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## taxa_get

> crate::models::TaxaShowResponse taxa_get(q, is_active, taxon_id, parent_id, rank, rank_level, id_above, id_below, per_page, locale, preferred_place_id, only_id, all_names, order, order_by)
Taxon Search

Given zero to many of following parameters, returns taxa matching the search criteria 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Name must begin with this value |  |
**is_active** | Option<**bool**> | Taxon is `active` |  |
**taxon_id** | Option<[**Vec<String>**](String.md)> | Only show taxa with this ID, or its descendants |  |
**parent_id** | Option<**i32**> | Taxon's parent must have this ID |  |
**rank** | Option<[**Vec<String>**](String.md)> | Taxon must have this rank |  |
**rank_level** | Option<**f32**> | Taxon must have this rank level. Some example values are 70 (kingdom), 60 (phylum), 50 (class), 40 (order), 30 (family), 20 (genus), 10 (species), 5 (subspecies)  |  |
**id_above** | Option<**String**> | Must have an ID above this value |  |
**id_below** | Option<**String**> | Must have an ID below this value |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**locale** | Option<**String**> | Locale preference for taxon common names  |  |
**preferred_place_id** | Option<**i32**> | Place preference for regional taxon common names  |  |
**only_id** | Option<**bool**> | Return only the record IDs |  |
**all_names** | Option<**bool**> | Include all taxon names in the response |  |
**order** | Option<**String**> | Sort order |  |[default to desc]
**order_by** | Option<**String**> | Sort field |  |[default to observations_count]

### Return type

[**crate::models::TaxaShowResponse**](TaxaShowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## taxa_id_get

> crate::models::TaxaShowResponse taxa_id_get(id)
Taxon Details

Given an ID, or an array of IDs in comma-delimited format, returns corresponding taxa. A maximum of 30 results will be returned 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<i32>**](i32.md) | Must have this ID | [required] |

### Return type

[**crate::models::TaxaShowResponse**](TaxaShowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

