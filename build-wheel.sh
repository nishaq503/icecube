#!/bin/bash

# maturin build --release --strip

version=$(grep -m 1 version Cargo.toml | tr -s ' ' | tr -d '"' | tr -d "'" | cut -d' ' -f3)

docker build . -t icecube:"${version}"

containerId=$(docker ps -aqf "name=icecube")
docker cp "${containerId}":/icecube/target/wheels .
