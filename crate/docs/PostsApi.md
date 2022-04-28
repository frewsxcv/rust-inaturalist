# \PostsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**posts_for_user_get**](PostsApi.md#posts_for_user_get) | **GET** /posts/for_user | Posts For User
[**posts_get**](PostsApi.md#posts_get) | **GET** /posts | Posts Search
[**posts_id_delete**](PostsApi.md#posts_id_delete) | **DELETE** /posts/{id} | Post Delete
[**posts_id_put**](PostsApi.md#posts_id_put) | **PUT** /posts/{id} | Post Update
[**posts_post**](PostsApi.md#posts_post) | **POST** /posts | Post Create



## posts_for_user_get

> posts_for_user_get(newer_than, older_than, page)
Posts For User

Return journal posts from the iNaturalist site. If the user is logged-in, also return posts from projects the user is subscribed to 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**newer_than** | Option<**String**> | returns posts newer than the post with this ID |  |
**older_than** | Option<**String**> | returns posts older than the post with this ID |  |
**page** | Option<**String**> | Pagination `page` number |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_get

> posts_get(login, project_id, page, per_page)
Posts Search

Return journal posts from the iNaturalist site 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login** | Option<**String**> | Return posts by this user |  |
**project_id** | Option<**i32**> | Return posts from this project |  |
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


## posts_id_delete

> posts_id_delete(id)
Post Delete

Delete a post 

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


## posts_id_put

> posts_id_put(id, body)
Post Update

Update a post 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PostPost**](PostPost.md)> | Post object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## posts_post

> posts_post(body)
Post Create

Create a post 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostPost**](PostPost.md)> | Post object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

