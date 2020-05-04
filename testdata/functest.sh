#!/usr/bin/env bash

set -xv

cargo build || exit 1

mkdir -p tests
cp -v ./testdata/bad.yaml ./tests/bad.yaml
cp -v ./testdata/bad.yaml ./tests/inplace.yaml
cp -v ./testdata/bad.yaml ./tests/inplace_w_bak.yaml

echo "test stdout"
./target/debug/ymlfxr ./tests/bad.yaml > ./tests/good.yaml
yamllint -s ./tests/good.yaml || exit 1
echo "test --fix"
./target/debug/ymlfxr --fix ./tests/inplace.yaml
yamllint -s ./tests/inplace.yaml || exit 1
echo "test --bak --fix"
./target/debug/ymlfxr --bak --fix ./tests/inplace_w_bak.yaml
yamllint -s ./tests/inplace_w_bak.yaml || exit 1
echo "testing the backup file expect errors" 
yamllint -s ./tests/inplace_w_bak.yaml.bak && exit 1
rm -rfv ./tests
