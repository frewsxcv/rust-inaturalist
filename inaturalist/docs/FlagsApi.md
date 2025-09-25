# \FlagsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**flags_id_delete**](FlagsApi.md#flags_id_delete) | **DELETE** /flags/{id} | Flag Delete
[**flags_id_put**](FlagsApi.md#flags_id_put) | **PUT** /flags/{id} | Flag Update
[**flags_post**](FlagsApi.md#flags_post) | **POST** /flags | Flag Create



## flags_id_delete

> flags_id_delete(id)
Flag Delete

Delete a flag 

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


## flags_id_put

> flags_id_put(id, body)
Flag Update

Update a flag. Generally only used to resolve the flag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PutFlag**](PutFlag.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flags_post

> flags_post(body)
Flag Create

Create a flag. To create a custom flag beyond the standard `spam` and `inappropriate` flags, set `flag` to `other` and include a `flag_explanation` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostFlag**](PostFlag.md)> | Flag object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

