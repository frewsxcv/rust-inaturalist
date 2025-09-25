# PostUserUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**login** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**name** | Option<**String**> | Display name for this user | [optional]
**locale** | Option<**String**> | Locale code for language/region localization. See https://github.com/inaturalist/inaturalist/tree/master/config/locales for available locales. Valid strings can be derived from file names, e.g. `es-MX` from `es-MX.yml`.  | [optional]
**time_zone** | Option<**String**> | Default time zone for the user's observations. See http://api.rubyonrails.org/classes/ActiveSupport/TimeZone.html for a list of values.  | [optional]
**place_id** | Option<**i32**> | ID of the place for this user, customizes some common names and default search parameters  | [optional]
**description** | Option<**String**> | User profile description | [optional]
**icon** | Option<[**serde_json::Value**](.md)> | User's profile pic. Requires POST/PUT as a multipart request.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


