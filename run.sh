#!/bin/sh
# docker run -p 3000:3000 --rm rust-tutorial:latest
docker-compose up && docker-compose rm --force
