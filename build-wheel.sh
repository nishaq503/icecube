#!/bin/bash

# maturin build --release --strip

version=$(grep -m 1 version Cargo.toml | tr -s ' ' | tr -d '"' | tr -d "'" | cut -d' ' -f3)

docker build . -t icecube:"${version}"

docker run --mount type=bind,source="$(pwd)",target=/wheels \
            --user "$(id -u)":"$(id -g)" \
            icecube:"${version}"
