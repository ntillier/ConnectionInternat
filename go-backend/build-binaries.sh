#!/usr/bin/env bash
# exit on error
set -o errexit
set -o pipefail
        set -o nounset
set -e

# Check if the crypto/tls/tls.bak folder exists
if [ ! -d "$(go1.8.1 env GOROOT)/src/crypto/tls.bak" ]; then
    echo "Crypto/tls/bak doesnt exist, exiting as it's probably the wrong tls version"
    exit 1
fi

rm -rf ./binaries

mkdir -p ./binaries

echo "Building amd64-linux"
GOOS=linux GOARCH=amd64 go1.8.1 build -o ./binaries/back-linux-amd64 -a main.go

echo "Building arm64-linux"
GOOS=linux GOARCH=arm64 go1.8.1 build -o ./binaries/back-linux-arm64 -a main.go

echo "Building amd64-darwin"
GOOS=darwin GOARCH=amd64 go1.8.1 build -o ./binaries/back-darwin-amd64 -a main.go

# echo "Building arm64-darwin"
# GOOS=darwin GOARCH=arm64 go1.8.1 build -o ./binaries/back-darwin-arm64 -a main.go

echo "Building amd64-windows"
GOOS=windows GOARCH=amd64 go1.8.1 build -o ./binaries/back-windows-amd64.exe -a main.go

