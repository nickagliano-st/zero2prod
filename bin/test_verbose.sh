#!/usr/bin/env bash
set -x

TEST_LOG=true cargo test | bunyan
