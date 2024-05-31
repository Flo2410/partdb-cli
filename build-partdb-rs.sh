#! /usr/bin/env bash

curl -H "Authorization: Bearer $1" https://partdb.hye.sh/api/docs.json -o ./partdb-api-docs.json
openapi-generator-cli generate -i ./partdb-api-docs.json -g rust -o ./partdb-rs -c ./openapi-config.yaml
rm ./partdb-api-docs.json