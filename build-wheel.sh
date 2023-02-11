#!/bin/bash

# maturin build --release --strip

cp .gitignore .dockerignore

rm -rf icecube/__pycache__ icecube/icecube.cpython-37m-x86_64-linux-gnu.so

version=$(grep -m 1 version Cargo.toml | tr -s ' ' | tr -d '"' | tr -d "'" | cut -d' ' -f3)

docker build . -t icecube:"${version}"

docker run --mount type=bind,source="$(pwd)",target=/wheels \
            --user "$(id -u)":"$(id -g)" \
            icecube:"${version}"

echo ""
echo "Built a wheel for kaggle!"
du -hs wheels/*
