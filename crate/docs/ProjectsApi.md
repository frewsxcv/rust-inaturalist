# \ProjectsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**projects_autocomplete_get**](ProjectsApi.md#projects_autocomplete_get) | **GET** /projects/autocomplete | Project Autocomplete
[**projects_get**](ProjectsApi.md#projects_get) | **GET** /projects | Project Search
[**projects_id_add_post**](ProjectsApi.md#projects_id_add_post) | **POST** /projects/{id}/add | Project Add
[**projects_id_get**](ProjectsApi.md#projects_id_get) | **GET** /projects/{id} | Project Details
[**projects_id_join_post**](ProjectsApi.md#projects_id_join_post) | **POST** /projects/{id}/join | Projects Join
[**projects_id_leave_delete**](ProjectsApi.md#projects_id_leave_delete) | **DELETE** /projects/{id}/leave | Projects Leave
[**projects_id_members_get**](ProjectsApi.md#projects_id_members_get) | **GET** /projects/{id}/members | Project Members
[**projects_id_membership_get**](ProjectsApi.md#projects_id_membership_get) | **GET** /projects/{id}/membership | Membership of current user
[**projects_id_remove_delete**](ProjectsApi.md#projects_id_remove_delete) | **DELETE** /projects/{id}/remove | Project Add
[**projects_id_subscriptions_get**](ProjectsApi.md#projects_id_subscriptions_get) | **GET** /projects/{id}/subscriptions | Project Subscriptions
[**subscriptions_project_id_subscribe_post**](ProjectsApi.md#subscriptions_project_id_subscribe_post) | **POST** /subscriptions/project/{id}/subscribe | Project Subscribe



## projects_autocomplete_get

> crate::models::ProjectsResponse projects_autocomplete_get(q, id, not_id, lat, lng, place_id, radius, featured, noteworthy, site_id, rule_details, r#type, member_id, has_params, has_posts, per_page)
Project Autocomplete

Given an string, returns projects with titles starting with the search term 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Name must begin with this value | [required] |
**id** | Option<[**Vec<String>**](String.md)> | Must have this ID |  |
**not_id** | Option<[**Vec<String>**](String.md)> | Must not have this ID |  |
**lat** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**lng** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**place_id** | Option<[**Vec<String>**](String.md)> | Must be associated with this place |  |
**radius** | Option<**String**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius). Defaults to 500km  |  |
**featured** | Option<**String**> | Must be marked featured for the relevant site |  |
**noteworthy** | Option<**String**> | Must be marked noteworthy for the relevant site |  |
**site_id** | Option<**i32**> | Site ID that applies to `featured` and `noteworthy`. Defaults to the site of the authenticated user, or to the main iNaturalist site https://www.inaturalist.org  |  |
**rule_details** | Option<**String**> | Return more information about project rules, for example return a full taxon object instead of simply an ID  |  |
**r#type** | Option<[**Vec<String>**](String.md)> | Projects must be of this type |  |
**member_id** | Option<**i32**> | Project must have member with this user ID |  |
**has_params** | Option<**bool**> | Must have search parameter requirements |  |
**has_posts** | Option<**bool**> | Must have posts |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |

### Return type

[**crate::models::ProjectsResponse**](ProjectsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_get

> crate::models::ProjectsResponse projects_get(q, id, not_id, lat, lng, place_id, radius, featured, noteworthy, site_id, rule_details, r#type, member_id, has_params, has_posts, per_page, order_by)
Project Search

Given zero to many of following parameters, returns projects matching the search criteria 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Name must begin with this value |  |
**id** | Option<[**Vec<String>**](String.md)> | Must have this ID |  |
**not_id** | Option<[**Vec<String>**](String.md)> | Must not have this ID |  |
**lat** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**lng** | Option<**f64**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius)  |  |
**place_id** | Option<[**Vec<String>**](String.md)> | Must be associated with this place |  |
**radius** | Option<**String**> | Must be within a {`radius`} kilometer circle around this lat/lng (*lat, *lng, radius). Defaults to 500km  |  |
**featured** | Option<**String**> | Must be marked featured for the relevant site |  |
**noteworthy** | Option<**String**> | Must be marked noteworthy for the relevant site |  |
**site_id** | Option<**i32**> | Site ID that applies to `featured` and `noteworthy`. Defaults to the site of the authenticated user, or to the main iNaturalist site https://www.inaturalist.org  |  |
**rule_details** | Option<**String**> | Return more information about project rules, for example return a full taxon object instead of simply an ID  |  |
**r#type** | Option<[**Vec<String>**](String.md)> | Projects must be of this type |  |
**member_id** | Option<**i32**> | Project must have member with this user ID |  |
**has_params** | Option<**bool**> | Must have search parameter requirements |  |
**has_posts** | Option<**bool**> | Must have posts |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |
**order_by** | Option<**String**> | Sort field |  |

### Return type

[**crate::models::ProjectsResponse**](ProjectsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_id_add_post

> projects_id_add_post(id, body)
Project Add

Add an observation to a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PostProjectAdd**](PostProjectAdd.md)> | ProjectObservation object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_id_get

> crate::models::ProjectsResponse projects_id_get(id, rule_details)
Project Details

Given an ID, or an array of IDs in comma-delimited format, returns corresponding projects. A maximum of 100 results will be returned 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<String>**](String.md) | Must have this ID or slug | [required] |
**rule_details** | Option<**String**> | Return more information about project rules, for example return a full taxon object instead of simply an ID  |  |

### Return type

[**crate::models::ProjectsResponse**](ProjectsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_id_join_post

> projects_id_join_post(id)
Projects Join

Join a project 

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


## projects_id_leave_delete

> projects_id_leave_delete(id)
Projects Leave

Leave a project 

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


## projects_id_members_get

> crate::models::ProjectMembersResponse projects_id_members_get(id, role, page, per_page)
Project Members

Given an ID, return members of the project 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**role** | Option<**String**> | Membership role |  |
**page** | Option<**String**> | Pagination `page` number |  |
**per_page** | Option<**String**> | Number of results to return in a `page`. The maximum value is generally 200 unless otherwise noted  |  |

### Return type

[**crate::models::ProjectMembersResponse**](ProjectMembersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_id_membership_get

> projects_id_membership_get(id)
Membership of current user

Given a project ID, return the details of the authenticated user's membership in that project 

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


## projects_id_remove_delete

> projects_id_remove_delete(id, body)
Project Add

Remove an observation from a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PostProjectAdd**](PostProjectAdd.md)> | ProjectObservation object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_id_subscriptions_get

> projects_id_subscriptions_get(id)
Project Subscriptions

[Deprecated] Subscriptions to projects are managed through joining and leaving projects, so this will not return any useful information.  Given an ID, return subscription of the current user 

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


## subscriptions_project_id_subscribe_post

> subscriptions_project_id_subscribe_post(id)
Project Subscribe

Toggles current user's subscription to this project. If the logged-in user is not subscribed, POSTing here will subscribe them. If they are already subscribed, this will remove the subscription 

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

