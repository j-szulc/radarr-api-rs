# \DelayProfileApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_delayprofile_get**](DelayProfileApi.md#api_v3_delayprofile_get) | **GET** /api/v3/delayprofile | 
[**api_v3_delayprofile_id_delete**](DelayProfileApi.md#api_v3_delayprofile_id_delete) | **DELETE** /api/v3/delayprofile/{id} | 
[**api_v3_delayprofile_id_get**](DelayProfileApi.md#api_v3_delayprofile_id_get) | **GET** /api/v3/delayprofile/{id} | 
[**api_v3_delayprofile_id_put**](DelayProfileApi.md#api_v3_delayprofile_id_put) | **PUT** /api/v3/delayprofile/{id} | 
[**api_v3_delayprofile_post**](DelayProfileApi.md#api_v3_delayprofile_post) | **POST** /api/v3/delayprofile | 



## api_v3_delayprofile_get

> Vec<crate::models::DelayProfileResource> api_v3_delayprofile_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DelayProfileResource>**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_delayprofile_id_delete

> api_v3_delayprofile_id_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_delayprofile_id_get

> crate::models::DelayProfileResource api_v3_delayprofile_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::DelayProfileResource**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_delayprofile_id_put

> crate::models::DelayProfileResource api_v3_delayprofile_id_put(id, delay_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**delay_profile_resource** | Option<[**DelayProfileResource**](DelayProfileResource.md)> |  |  |

### Return type

[**crate::models::DelayProfileResource**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_delayprofile_post

> crate::models::DelayProfileResource api_v3_delayprofile_post(delay_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delay_profile_resource** | Option<[**DelayProfileResource**](DelayProfileResource.md)> |  |  |

### Return type

[**crate::models::DelayProfileResource**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

