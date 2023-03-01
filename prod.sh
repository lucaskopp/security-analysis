#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

pushd frontend
# note: when using SpaRouter this needs to be
#   "trunk build --public-url /"
trunk build
popd

cargo run --bin server --release -- --port 8080 --static-dir ./dist