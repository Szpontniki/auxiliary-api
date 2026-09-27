#!/bin/sh

openapi-generator-cli generate -i ./api-schema/auxiliary/openapi.yaml -g rust -o ./openapi
