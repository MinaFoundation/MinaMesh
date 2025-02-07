#!/bin/bash

APP=$1

if [ $APP == "mesh" ]; then
    BASE_URL="http://localhost:3000/construction/payloads"
elif [ $APP == "ros" ]; then
    # BASE_URL="http://localhost:4000/construction/payloads"
    BASE_URL="https://rosetta-devnet.minaprotocol.network/construction/payloads"
else
    echo "Invalid app name. Please provide either 'mesh' or 'ros'."
    exit 1
fi



HEADERS=(-H "Content-Type: application/json" -H "Accept: application/json")

QUERIES=(
    '{
  "network_identifier": {
	"blockchain": "mina",
	"network": "testnet"
  },
  "operations": [
	{
	  "operation_identifier": {
		"index": 0
	  },
	  "related_operations": [],
	  "type": "fee_payment",
	  "account": {
		"address": "B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk",
		"metadata": {
		  "token_id": "1"
		}
	  },
	  "amount": {
		"value": "-100000",
		"currency": {
		  "symbol": "MINA",
		  "decimals": 9
		}
	  }
	},
	{
	  "operation_identifier": {
		"index": 1
	  },
	  "related_operations": [],
	  "type": "payment_source_dec",
	  "account": {
		"address": "B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk",
		"metadata": {
		  "token_id": "1"
		}
	  },
	  "amount": {
		"value": "-5000000000",
		"currency": {
		  "symbol": "MINA",
		  "decimals": 9
		}
	  }
	},
	{
	  "operation_identifier": {
		"index": 2
	  },
	  "related_operations": [
		{
		  "index": 1
		}
	  ],
	  "type": "payment_receiver_inc",
	  "account": {
		"address": "B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv",
		"metadata": {
		  "token_id": "1"
		}
	  },
	  "amount": {
		"value": "5000000000",
		"currency": {
		  "symbol": "MINA",
		  "decimals": 9
		}
	  }
	}
  ],
  "metadata": {
	"sender": "B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk",
	"nonce": "1984",
	"token_id": "1",
	"receiver": "B62qoDWfBZUxKpaoQCoFqr12wkaY84FrhxXNXzgBkMUi2Tz4K8kBDiv",
	"memo": "hello",
	"account_creation_fee":"1000000"
  }
}'
# '{
#   "network_identifier": {
# 	"blockchain": "mina",
# 	"network": "testnet"
#   },
# "operations": [
# 		  {
# 			"operation_identifier": {
# 			  "index": 0
# 			},
# 			"type": "fee_payment",
# 			"account": {
# 			  "address": "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB"
# 			},
# 			"amount": {
# 			  "value": "-10100000",
# 			  "currency": {
# 				"symbol": "MINA",
# 				"decimals": 9
# 			  }
# 			}
# 		  },
# 		  {
# 			"operation_identifier": {
# 			  "index": 1
# 			},
# 			"type": "delegate_change",
# 			"account": {
# 			  "address": "B62qkXajxfnicuCNtaurdAhQpkFsqjoyPJuw53aeJP848bsa3Ne3RvB"
# 			},
# 			"metadata": {
# 			  "delegate_change_target": "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X"
# 			}
# 		  }
# 		],
#   "metadata": {
# 	"sender": "B62qkUHaJUHERZuCHQhXCQ8xsGBqyYSgjQsKnKN5HhSJecakuJ4pYyk",
# 	"nonce": "3",
# 	"token_id": "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf",
# 	"receiver": "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X",
# 	"valid_until": "200000",
# 	"memo": "hello"
#   }
# }'
)

REPORT="Execution Times Report\n========================\n\n"

for i in "${!QUERIES[@]}"; do
    QUERY="${QUERIES[$i]}"
    echo "Running query $((i+1))..."

    RESPONSE=$(curl -s -o ./construct_resp.json -w "%{http_code}" -L -X POST "$BASE_URL" "${HEADERS[@]}" -d "$QUERY")
    cat ./construct_resp.json | jq
    rm ./construct_resp.json
done
