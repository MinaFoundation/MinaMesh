# AccountBalanceRequest

## Properties

| Name                   | Type                                                                    | Description                                                                                                                                                                                                                                                       | Notes      |
| ---------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| **network_identifier** | [**models::NetworkIdentifier**](NetworkIdentifier.md)                   |                                                                                                                                                                                                                                                                   |            |
| **account_identifier** | [**models::AccountIdentifier**](AccountIdentifier.md)                   |                                                                                                                                                                                                                                                                   |            |
| **block_identifier**   | Option<[**models::PartialBlockIdentifier**](PartialBlockIdentifier.md)> |                                                                                                                                                                                                                                                                   | [optional] |
| **currencies**         | Option<[**Vec<models::Currency>**](Currency.md)>                        | In some cases, the caller may not want to retrieve all available balances for an AccountIdentifier. If the currencies field is populated, only balances for the specified currencies will be returned. If not populated, all available balances will be returned. | [optional] |

[[Back to Model list]](../README.md#documentation-for-models)
[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
