# μDCN Persistent Daemon IPC

The daemon listens on a Unix domain socket at `/tmp/udcn.sock` and accepts simple
newline delimited commands. Each connection is handled independently. Commands
and responses are UTF‑8 text.

## Commands

- `fib add <prefix> <face>` – Insert a forwarding entry mapping a name prefix to
a face identifier. Returns `OK` on success.
- `fib list` – List all FIB entries in the form `<prefix> -> <face>`.
- `cs stats` – Return the current number of entries stored in the content
  store.
- `quit` – Close the connection.

Unknown commands result in `UNKNOWN`.


