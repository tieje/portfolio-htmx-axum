#!/bin/bash
cargo watch -s "npm run build && cargo r" -w templates -w data.json -w src -w tailwind.config.js -w styles/base.scss
