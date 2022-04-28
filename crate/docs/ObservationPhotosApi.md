# \ObservationPhotosApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**observation_photos_id_delete**](ObservationPhotosApi.md#observation_photos_id_delete) | **DELETE** /observation_photos/{id} | Observation Photo Delete
[**observation_photos_id_put**](ObservationPhotosApi.md#observation_photos_id_put) | **PUT** /observation_photos/{id} | Observation Photo Update
[**observation_photos_post**](ObservationPhotosApi.md#observation_photos_post) | **POST** /observation_photos | Observation Photo Create



## observation_photos_id_delete

> observation_photos_id_delete(id)
Observation Photo Delete

Delete an observation photo 

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


## observation_photos_id_put

> observation_photos_id_put(id, observation_photo_position, file)
Observation Photo Update

Update an observation photo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |
**observation_photo_position** | Option<**i32**> | Position in which the photo is displayed for the observation |  |
**file** | Option<**std::path::PathBuf**> | The photo |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## observation_photos_post

> observation_photos_post(observation_photo_observation_id, observation_photo_uuid, file)
Observation Photo Create

Create an observation photo 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**observation_photo_observation_id** | Option<**i32**> | Observation ID |  |
**observation_photo_uuid** | Option<**String**> | Observation UUID |  |
**file** | Option<**std::path::PathBuf**> | The photo |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

