# \ImportExclusionsApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_exclusions_bulk_post**](ImportExclusionsApi.md#api_v3_exclusions_bulk_post) | **POST** /api/v3/exclusions/bulk | 
[**api_v3_exclusions_get**](ImportExclusionsApi.md#api_v3_exclusions_get) | **GET** /api/v3/exclusions | 
[**api_v3_exclusions_id_delete**](ImportExclusionsApi.md#api_v3_exclusions_id_delete) | **DELETE** /api/v3/exclusions/{id} | 
[**api_v3_exclusions_id_get**](ImportExclusionsApi.md#api_v3_exclusions_id_get) | **GET** /api/v3/exclusions/{id} | 
[**api_v3_exclusions_id_put**](ImportExclusionsApi.md#api_v3_exclusions_id_put) | **PUT** /api/v3/exclusions/{id} | 
[**api_v3_exclusions_post**](ImportExclusionsApi.md#api_v3_exclusions_post) | **POST** /api/v3/exclusions | 



## api_v3_exclusions_bulk_post

> api_v3_exclusions_bulk_post(import_exclusions_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_exclusions_resource** | Option<[**Vec<crate::models::ImportExclusionsResource>**](ImportExclusionsResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_get

> Vec<crate::models::ImportExclusionsResource> api_v3_exclusions_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ImportExclusionsResource>**](ImportExclusionsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_id_delete

> api_v3_exclusions_id_delete(id)


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


## api_v3_exclusions_id_get

> crate::models::ImportExclusionsResource api_v3_exclusions_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::ImportExclusionsResource**](ImportExclusionsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_id_put

> crate::models::ImportExclusionsResource api_v3_exclusions_id_put(id, import_exclusions_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**import_exclusions_resource** | Option<[**ImportExclusionsResource**](ImportExclusionsResource.md)> |  |  |

### Return type

[**crate::models::ImportExclusionsResource**](ImportExclusionsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_exclusions_post

> crate::models::ImportExclusionsResource api_v3_exclusions_post(import_exclusions_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_exclusions_resource** | Option<[**ImportExclusionsResource**](ImportExclusionsResource.md)> |  |  |

### Return type

[**crate::models::ImportExclusionsResource**](ImportExclusionsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

