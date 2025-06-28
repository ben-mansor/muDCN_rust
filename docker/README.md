# Docker Setup

The Docker files allow you to spin up a small testbed with a forwarder, client and server container. Each container uses the binaries built from this repository.

1. Build the images and start the containers:
   ```bash
   docker compose -f docker/docker-compose.yml build
   docker compose -f docker/docker-compose.yml up -d
   ```
2. Interact with the forwarder container using the `udcn` CLI, for example:
   ```bash
   docker compose -f docker/docker-compose.yml exec forwarder udcn fib add /demo
   docker compose -f docker/docker-compose.yml exec forwarder udcn stats
   ```
3. Shut everything down once done:
   ```bash
   docker compose -f docker/docker-compose.yml down
   ```

See [`../docs/testbed.md`](../docs/testbed.md) for the expected output of the demo script located in `demo/test_udcn.sh`.

