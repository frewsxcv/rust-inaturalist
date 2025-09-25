# \PolygonTilesApi

All URIs are relative to */v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**places_place_id_zoom_xy_png_get**](PolygonTilesApi.md#places_place_id_zoom_xy_png_get) | **GET** /places/{place_id}/{zoom}/{x}/{y}.png | Place Tiles
[**taxon_places_taxon_id_zoom_xy_png_get**](PolygonTilesApi.md#taxon_places_taxon_id_zoom_xy_png_get) | **GET** /taxon_places/{taxon_id}/{zoom}/{x}/{y}.png | Taxon Place Tiles
[**taxon_ranges_taxon_id_zoom_xy_png_get**](PolygonTilesApi.md#taxon_ranges_taxon_id_zoom_xy_png_get) | **GET** /taxon_ranges/{taxon_id}/{zoom}/{x}/{y}.png | Taxon Range Tiles



## places_place_id_zoom_xy_png_get

> places_place_id_zoom_xy_png_get(place_id, zoom, x, y, ttl)
Place Tiles

Returns a PNG map tile representing the boundary of this place, following the XYZ tiling scheme 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**place_id** | **i32** | Place ID | [required] |
**zoom** | **i32** | Zoom level. Z coordinate in the XYZ tiling scheme | [required] |
**x** | **i32** | X coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**y** | **i32** | Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**ttl** | Option<**String**> | Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## taxon_places_taxon_id_zoom_xy_png_get

> taxon_places_taxon_id_zoom_xy_png_get(taxon_id, zoom, x, y, ttl)
Taxon Place Tiles

Returns a PNG map tile representing the boundaries of places this taxon is known to occur, following the XYZ tiling scheme 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**taxon_id** | **i32** | Taxon ID | [required] |
**zoom** | **i32** | Zoom level. Z coordinate in the XYZ tiling scheme | [required] |
**x** | **i32** | X coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**y** | **i32** | Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**ttl** | Option<**String**> | Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## taxon_ranges_taxon_id_zoom_xy_png_get

> taxon_ranges_taxon_id_zoom_xy_png_get(taxon_id, zoom, x, y, color, ttl)
Taxon Range Tiles

Returns a PNG map tile representing the range of this taxon, following the XYZ tiling scheme 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**taxon_id** | **i32** | Taxon ID | [required] |
**zoom** | **i32** | Zoom level. Z coordinate in the XYZ tiling scheme | [required] |
**x** | **i32** | X coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**y** | **i32** | Y coordinate in the XYZ tiling scheme. Must be less than 2^zoom | [required] |
**color** | Option<**String**> | Primary color to use in tile creation. Accepts common colors by string (e.g. `color=blue`), and accepts escaped color HEX codes (e.g. `color=%2386a91c`)  |  |
**ttl** | Option<**String**> | Set the `Cache-Control` HTTP header with this value as `max-age`, in seconds. This means subsequent identical requests will be cached on iNaturalist servers, and commonly within web browsers  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

