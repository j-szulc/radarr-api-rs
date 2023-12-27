# \MovieImportApi

All URIs are relative to *http://localhost:7878*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_movie_import_id_get**](MovieImportApi.md#api_v3_movie_import_id_get) | **GET** /api/v3/movie/import/{id} | 
[**api_v3_movie_import_post**](MovieImportApi.md#api_v3_movie_import_post) | **POST** /api/v3/movie/import | 



## api_v3_movie_import_id_get

> crate::models::MovieResource api_v3_movie_import_id_get(id)


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


## api_v3_movie_import_post

> api_v3_movie_import_post(movie_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_resource** | Option<[**Vec<crate::models::MovieResource>**](MovieResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

