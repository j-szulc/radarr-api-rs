# \CalendarApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_calendar_get**](CalendarApi.md#api_v3_calendar_get) | **GET** /api/v3/calendar | 
[**api_v3_calendar_id_get**](CalendarApi.md#api_v3_calendar_id_get) | **GET** /api/v3/calendar/{id} | 



## api_v3_calendar_get

> Vec<crate::models::MovieResource> api_v3_calendar_get(start, end, unmonitored, tags)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**String**> |  |  |
**end** | Option<**String**> |  |  |
**unmonitored** | Option<**bool**> |  |  |[default to false]
**tags** | Option<**String**> |  |  |[default to ]

### Return type

[**Vec<crate::models::MovieResource>**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_calendar_id_get

> crate::models::MovieResource api_v3_calendar_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::MovieResource**](MovieResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

