# Demo Scripts

The `test_udcn.sh` script launches the Docker testbed defined in `docker/docker-compose.yml` and runs a few CLI commands inside the forwarder container. It is a quick way to check that the binaries work together.

Run it from the repository root:
```bash
./demo/test_udcn.sh
```
The script builds the containers, adds a FIB entry and shows the forwarder's statistics before shutting everything down.

