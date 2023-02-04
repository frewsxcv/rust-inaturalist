# \MessagesApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**messages_get**](MessagesApi.md#messages_get) | **GET** /messages | Retrieve messages for the authenticated user. This does not mark them as read.
[**messages_id_delete**](MessagesApi.md#messages_id_delete) | **DELETE** /messages/{id} | Delete a message / thread
[**messages_id_get**](MessagesApi.md#messages_id_get) | **GET** /messages/{id} | Retrieve messages in a thread
[**messages_post**](MessagesApi.md#messages_post) | **POST** /messages | Create a new message
[**messages_unread_get**](MessagesApi.md#messages_unread_get) | **GET** /messages/unread | Gets a count of messages the authenticated user has not read



## messages_get

> messages_get(page, r#box, q, user_id, threads)
Retrieve messages for the authenticated user. This does not mark them as read.

Show the user's inbox or sent box

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**String**> | Pagination `page` number |  |
**r#box** | Option<**String**> | Whether to view messages the user has received (default) or messages the user has sent |  |[default to inbox]
**q** | Option<**String**> | Search query for subject and body |  |
**user_id** | Option<**String**> | User ID or username of correspondent to filter by |  |
**threads** | Option<**bool**> | Groups results by `thread_id`, only shows the latest message per thread, and includes a `thread_messages_count` attribute showing the total number of messages in that thread. Note that this will not work with the `q` param, and it probably should only be used with `box=any` because the `thread_messages_count` will be inaccurate when you restrict it to `inbox` or `sent`.  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## messages_id_delete

> messages_id_delete(id)
Delete a message / thread

This will all of the authenticated user's copies of the messages in tha thread to which the specified message belongs. 

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## messages_id_get

> serde_json::Value messages_id_get(id)
Retrieve messages in a thread

Retrieves all messages in the thread the specified message belongs to and marks them all as read. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the record | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## messages_post

> crate::models::Message messages_post(body)
Create a new message

Create and deliver a new message to another user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**PostMessage**](PostMessage.md)> |  |  |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## messages_unread_get

> serde_json::Value messages_unread_get()
Gets a count of messages the authenticated user has not read

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

