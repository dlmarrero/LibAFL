#!/bin/bash
set -e

docker build --network=host -t libafl:base -f Dockerfile .
docker build --network=host -t libafl:offline -f Dockerfile-offline .
docker build --network=host -t libafl:offline-vscode -f Dockerfile-vscode .

echo "Saving image tarball..."
docker save libafl | gzip > /tmp/libafl_offline.tgz

