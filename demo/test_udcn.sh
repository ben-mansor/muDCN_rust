#!/bin/bash
set -e
cd "$(dirname "$0")/.."
COMPOSE="docker/docker-compose.yml"
# build and start containers
docker compose -f "$COMPOSE" build
docker compose -f "$COMPOSE" up -d
sleep 2
# run a couple of CLI commands inside the forwarder container
docker compose -f "$COMPOSE" exec forwarder udcn fib add /demo
docker compose -f "$COMPOSE" exec forwarder udcn stats
# shutdown
docker compose -f "$COMPOSE" down
