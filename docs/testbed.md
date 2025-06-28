# Docker Testbed

This testbed launches three containers: `client`, `forwarder`, and `server`.
The client and server containers are idle shells while the forwarder runs
`udcn-daemon`.

Run the demo script:

```bash
./demo/test_udcn.sh
```

Expected output (abbreviated):

```
[+] Building ...
[+] Running 3/3
 ✔ Container udcn-rust-forwarder-1  Started
 ✔ Container udcn-rust-client-1     Started
 ✔ Container udcn-rust-server-1     Started
OK
CS entries: 0
Going to shut down containers...
```

The `OK` line confirms the FIB entry was added and `CS entries: 0` is returned
by the `udcn stats` command.
