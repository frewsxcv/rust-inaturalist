# \UsersApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_autocomplete_get**](UsersApi.md#users_autocomplete_get) | **GET** /users/autocomplete | User Autocomplete
[**users_id_get**](UsersApi.md#users_id_get) | **GET** /users/{id} | User Details
[**users_id_mute_delete**](UsersApi.md#users_id_mute_delete) | **DELETE** /users/{id}/mute | Unmute a User
[**users_id_mute_post**](UsersApi.md#users_id_mute_post) | **POST** /users/{id}/mute | Mute a User
[**users_id_projects_get**](UsersApi.md#users_id_projects_get) | **GET** /users/{id}/projects | User Projects
[**users_id_put**](UsersApi.md#users_id_put) | **PUT** /users/{id} | User Update
[**users_me_get**](UsersApi.md#users_me_get) | **GET** /users/me | Users Me
[**users_update_session_put**](UsersApi.md#users_update_session_put) | **PUT** /users/update_session | User Update Session



## users_autocomplete_get

> users_autocomplete_get(q, project_id, per_page)
User Autocomplete

Given an string, returns users with names or logins starting with the search term 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Name must begin with this value | [required] |
**project_id** | Option<**i32**> | Only show users with memberships to this project |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_id_get

> users_id_get(id)
User Details

Given an ID, returns corresponding user

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


## users_id_mute_delete

> users_id_mute_delete(id)
Unmute a User

Remove a mute on the user specified by {id}

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


## users_id_mute_post

> users_id_mute_post(id)
Mute a User

Make it so the authenticated user stops receiving notifications about activity by the user specified by {id}. 

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


## users_id_projects_get

> users_id_projects_get(id, rule_details, project_type, page, per_page)
User Projects

Return projects as user has joined / followed 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**rule_details** | Option<**String**> | Return more information about project rules, for example return a full taxon object instead of simply an ID  |  |
**project_type** | Option<**String**> | Specify the type of project to return  |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_id_put

> users_id_put(id)
User Update

Update a user 

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


## users_me_get

> users_me_get()
Users Me

Fetch the logged-in user

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update_session_put

> users_update_session_put(body)
User Update Session

Update the logged-in user's session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostUserUpdateSession**](PostUserUpdateSession.md)> | Comment object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

