# TaxonConservationStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_id** | Option<**i32**> | Identifier for the iNat source record associated with this status, retrievable via https://www.inaturalist.org/sources/:id.json (this endpoint is not a part of our public API and is thus subject to change or removal)  | [optional]
**authority** | Option<**String**> | Organization that declared this status  | [optional]
**status** | Option<**String**> | Body of the status, often coded, particularly when the status comes from the IUCN or NatureServe. Consult the authority and/or the status URL for details about the meanings of codes.  | [optional]
**status_name** | Option<**String**> | Human-readable name of the status if it was coded.  | [optional]
**iucn** | Option<**i32**> | Coded value representing the equivalent IUCN status. Mappings: NOT_EVALUATED = 0, DATA_DEFICIENT = 5, LEAST_CONCERN = 10, NEAR_THREATENED = 20, VULNERABLE = 30, ENDANGERED = 40, CRITICALLY_ENDANGERED = 50, EXTINCT_IN_THE_WILD = 60, EXTINCT = 70  | [optional]
**geoprivacy** | Option<**String**> | Default geoprivacy for observations of this taxon in the status's place.  | [optional]
**place** | Option<[**models::CorePlace**](CorePlace.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


