# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**subject** | Option<**String**> |  | [optional]
**body** | Option<**String**> |  | [optional]
**user_id** | Option<**i32**> | ID of the user to whom this message belongs. Messages work like email, so the sender gets a copy and the recipient gets a copy of each message. This is always the authenticated user, so there's no real need for a full user object.  | [optional]
**to_user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**from_user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**thread_id** | Option<**i32**> | Identifier for the message thread, generally the ID of the sender's copy of the first message  | [optional]
**thread_messages_count** | Option<**i32**> | Number of messages in this thread. Only included when threads=true  | [optional]
**thread_flags** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array of flags on messages in this thread. Only included when threads=true  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


