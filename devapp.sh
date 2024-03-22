#!/bin/bash

cargo watch -x run \
  -w app/ \
  -w templates/ \
  -w Cargo.toml \
  -w Cargo.lock \
  -w .env
