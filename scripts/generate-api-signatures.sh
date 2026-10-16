#!/bin/sh

cd "$(dirname "$0")" || exit 1

openapi-generator-cli generate -i ../api-schema/auxiliary/openapi.yaml -g rust -o ../openapi
