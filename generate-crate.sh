#!/bin/bash

curl https://api.inaturalist.org/v1/swagger.json > swagger.json
rm -rf crate
openapi-generator generate -i swagger.json -g rust -o ./crate -c config.json
rm swagger.json

