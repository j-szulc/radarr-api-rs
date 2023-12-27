# \HealthApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_health_get**](HealthApi.md#api_v3_health_get) | **GET** /api/v3/health | 
[**api_v3_health_id_get**](HealthApi.md#api_v3_health_id_get) | **GET** /api/v3/health/{id} | 



## api_v3_health_get

> Vec<crate::models::HealthResource> api_v3_health_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::HealthResource>**](HealthResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_health_id_get

> crate::models::HealthResource api_v3_health_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::HealthResource**](HealthResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

