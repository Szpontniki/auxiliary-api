#!/bin/sh

cd "$(dirname "$0")" || exit 1

source ../.env

databse_url=postgres://$DATABASE_USER:$DATABASE_PASSWORD@$DATABASE_URI:$DATABASE_PORT\/$DATABASE_NAME
DATABASE_URL=$databse_url diesel migration run && DATABASE_URL=$databse_url diesel setup
