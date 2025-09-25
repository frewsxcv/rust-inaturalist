# \CommentsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**comments_id_delete**](CommentsApi.md#comments_id_delete) | **DELETE** /comments/{id} | Comment Delete
[**comments_id_put**](CommentsApi.md#comments_id_put) | **PUT** /comments/{id} | Comment Update
[**comments_post**](CommentsApi.md#comments_post) | **POST** /comments | Comment Create



## comments_id_delete

> comments_id_delete(id)
Comment Delete

Delete a comment 

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


## comments_id_put

> comments_id_put(id, body)
Comment Update

Update a comment 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PostComment**](PostComment.md)> | Comment object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comments_post

> comments_post(body)
Comment Create

Create a comment 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostComment**](PostComment.md)> | Comment object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

