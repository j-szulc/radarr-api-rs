# \QueueStatusApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_queue_status_get**](QueueStatusApi.md#api_v3_queue_status_get) | **GET** /api/v3/queue/status | 
[**api_v3_queue_status_id_get**](QueueStatusApi.md#api_v3_queue_status_id_get) | **GET** /api/v3/queue/status/{id} | 



## api_v3_queue_status_get

> crate::models::QueueStatusResource api_v3_queue_status_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::QueueStatusResource**](QueueStatusResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_queue_status_id_get

> crate::models::QueueStatusResource api_v3_queue_status_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::QueueStatusResource**](QueueStatusResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

