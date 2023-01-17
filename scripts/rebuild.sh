#!/usr/bin/env bash
cargo build --bin fedimintd
pkill -9 fedimintd
./scripts/start-fed.sh
