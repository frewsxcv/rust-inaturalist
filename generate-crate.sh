#!/bin/bash

curl https://api.inaturalist.org/v1/swagger.json > swagger.json
rm -rf crate
openapi-generator generate -i swagger.json -g rust -o ./inaturalist -c config.json
rm swagger.json
(cd inaturalist && cargo fmt)
