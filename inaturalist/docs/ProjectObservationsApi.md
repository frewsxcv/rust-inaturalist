# \ProjectObservationsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**project_observations_id_delete**](ProjectObservationsApi.md#project_observations_id_delete) | **DELETE** /project_observations/{id} | Project Observation Delete
[**project_observations_id_put**](ProjectObservationsApi.md#project_observations_id_put) | **PUT** /project_observations/{id} | Project Observation Update
[**project_observations_post**](ProjectObservationsApi.md#project_observations_post) | **POST** /project_observations | Project Observation Create



## project_observations_id_delete

> project_observations_id_delete(id)
Project Observation Delete

Delete a project observation

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


## project_observations_id_put

> project_observations_id_put(id, body)
Project Observation Update

Update a project observation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**UpdateProjectObservation**](UpdateProjectObservation.md)> | Comment object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_observations_post

> project_observations_post(body)
Project Observation Create

Add an observation to a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostProjectObservation**](PostProjectObservation.md)> | ProjectObservation object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

