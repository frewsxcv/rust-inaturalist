# \ObservationFieldValuesApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**observation_field_values_id_delete**](ObservationFieldValuesApi.md#observation_field_values_id_delete) | **DELETE** /observation_field_values/{id} | Observation Field Value Delete
[**observation_field_values_id_put**](ObservationFieldValuesApi.md#observation_field_values_id_put) | **PUT** /observation_field_values/{id} | Observation Field Value Update
[**observation_field_values_post**](ObservationFieldValuesApi.md#observation_field_values_post) | **POST** /observation_field_values | Observation Field Value Create



## observation_field_values_id_delete

> observation_field_values_id_delete(id)
Observation Field Value Delete

Delete an observation field value 

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


## observation_field_values_id_put

> observation_field_values_id_put(id, body)
Observation Field Value Update

Update an observation field value 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**body** | Option<[**PostObservationFieldValue**](PostObservationFieldValue.md)> | Observation field value object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observation_field_values_post

> observation_field_values_post(body)
Observation Field Value Create

Create an observation field value 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostObservationFieldValue**](PostObservationFieldValue.md)> | Observation field value object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

