#!/bin/bash

# Before publishing, we need to download the cw-plus artifacts
./before_publish.sh

# Before publishing, test the version with the Abstract implementation to make sure you're not breaking important API
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

function print_usage() {
  echo "Usage: $0 [-h|--help]"
  echo "Publishes crates to crates.io."
}

if [ $# = 1 ] && { [ "$1" = "-h" ] || [ "$1" = "--help" ] ; }
then
    print_usage
    exit 1
fi

# these are imported by other packages
BASE_PACKAGES="
  cw-orch-contract-derive 
  cw-orch-fns-derive
  cw-orch-core
  cw-orch-traits  
  cw-orch-mock 
  cw-orch-networks  
"

INTERCHAIN_PACKAGES="
  interchain-core
  starship
  interchain-daemon
  interchain-mock
"

CORE="cw-orch-daemon cw-orch cw-orch-interchain"

INTEGRATIONS="
  cw-plus
"

for pack in $BASE_PACKAGES; do
  (
    cd "packages/$pack"
    echo "Publishing $pack"
    cargo publish
  )
done

for lib in $INTERCHAIN; do
  (
    cd "packages/interchain/$lib"
    echo "Publishing $lib"
    cargo publish
  )
done

for lib in $CORE; do
  (
    cd "$lib"
    echo "Publishing $lib"
    cargo publish
  )
done

for integration in $INTEGRATIONS; do
  (
    cd "packages/integrations/$integration"
    echo "Publishing $integration"
    cargo publish
  )
done

echo "Everything is published!"

# 
# VERSION=$(< Cargo.toml grep -m 1 version | sed 's/-/_/g' | grep -o '".*"' | sed 's/"//g');
# git push
# git tag v"$VERSION"
# git push origin v"$VERSION"