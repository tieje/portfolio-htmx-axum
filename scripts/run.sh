#!/bin/bash
cargo watch -s "npm run build && cargo r" -w templates -w data/portfolio.json -w data/blog.json -w data/content/ -w src -w tailwind.config.js -w styles/base.scss -w assets/blog.css
