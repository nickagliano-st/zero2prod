#!/usr/bin/env bash
set -x
set -eo pipefail

docker stop postgres
docker rm postgres