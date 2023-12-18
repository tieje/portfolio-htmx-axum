#!/bin/bash
cargo watch -s "npm run build && cargo r" -w templates -w src
