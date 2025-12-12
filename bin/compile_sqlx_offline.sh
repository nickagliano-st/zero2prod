#!/usr/bin/env bash
set -x

cargo sqlx prepare --workspace -- --all-targets
