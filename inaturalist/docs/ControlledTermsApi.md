# \ControlledTermsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**controlled_terms_for_taxon_get**](ControlledTermsApi.md#controlled_terms_for_taxon_get) | **GET** /controlled_terms/for_taxon | Terms for Taxon
[**controlled_terms_get**](ControlledTermsApi.md#controlled_terms_get) | **GET** /controlled_terms | Terms Index



## controlled_terms_for_taxon_get

> controlled_terms_for_taxon_get(taxon_id)
Terms for Taxon

Returns attribute controlled terms relevant to a taxon 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**taxon_id** | **i32** | Filter by this taxon | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## controlled_terms_get

> controlled_terms_get()
Terms Index

List all attribute controlled terms 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

