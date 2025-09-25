# \AnnotationsApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**annotations_id_delete**](AnnotationsApi.md#annotations_id_delete) | **DELETE** /annotations/{id} | Annotation Delete
[**annotations_post**](AnnotationsApi.md#annotations_post) | **POST** /annotations | Annotation Create
[**votes_unvote_annotation_id_delete**](AnnotationsApi.md#votes_unvote_annotation_id_delete) | **DELETE** /votes/unvote/annotation/{id} | Annotation Unvote
[**votes_vote_annotation_id_post**](AnnotationsApi.md#votes_vote_annotation_id_post) | **POST** /votes/vote/annotation/{id} | Annotation Vote



## annotations_id_delete

> annotations_id_delete(id)
Annotation Delete

Delete an annotation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or UUID of the annotation | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## annotations_post

> annotations_post(body)
Annotation Create

Create an annotation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostAnnotation**](PostAnnotation.md)> | Annotation object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_unvote_annotation_id_delete

> votes_unvote_annotation_id_delete(id)
Annotation Unvote

Remove a vote from annotation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or UUID of the annotation | [required] |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_vote_annotation_id_post

> votes_vote_annotation_id_post(id, body)
Annotation Vote

Vote on an annotation 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID or UUID of the annotation | [required] |
**body** | Option<[**PostVote**](PostVote.md)> | Vote object |  |

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

