#!/bin/bash

rm -rf crate
openapi-generator generate -i inaturalist-swagger.json -g rust -o ./crate -c config.json
