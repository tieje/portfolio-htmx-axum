#!/bin/bash
docker build --platform=linux/arm64 -t tieje/portfolio:latest .
docker push tieje/portfolio:latest
