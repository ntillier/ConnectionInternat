#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export OPENSSL_CONF="$SCRIPT_DIR/openssl_config.cnf"

if [ "$2" == "GET" ]; then
    curl -k --tlsv1 "$1"
else
    curl -k --tlsv1 "$1" -X "$2" -H "$3" --data-raw "$4"
fi
