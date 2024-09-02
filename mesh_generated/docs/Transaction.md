# Transaction

## Properties

| Name                       | Type                                                                 | Description                                                                                                                                                          | Notes      |
| -------------------------- | -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| **transaction_identifier** | [**models::TransactionIdentifier**](TransactionIdentifier.md)        |                                                                                                                                                                      |            |
| **operations**             | [**Vec<models::Operation>**](Operation.md)                           |                                                                                                                                                                      |            |
| **related_transactions**   | Option<[**Vec<models::RelatedTransaction>**](RelatedTransaction.md)> |                                                                                                                                                                      | [optional] |
| **metadata**               | Option<[**serde_json::Value**](.md)>                                 | Transactions that are related to other transactions (like a cross-shard transaction) should include the tranaction_identifier of these transactions in the metadata. | [optional] |

[[Back to Model list]](../README.md#documentation-for-models)
[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
