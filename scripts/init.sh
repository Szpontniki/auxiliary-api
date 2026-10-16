#!/bin/sh

cd "$(dirname "$0")" || exit 1

./generate-api-signatures.sh && ./generate-database-signatures.sh
