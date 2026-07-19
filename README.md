# App visualize network connect

Network visualization project written in C. The current repository contains the
first packet-capture pieces and documentation for the planned GUI/API backend
integration.

## Current implementation

Verified from the files tracked in this repository:

- Language: C
- Packet capture dependency: libpcap/Npcap headers through `pcap/pcap.h`
- Implemented capture helper: `list_interface()` in `src/capture/pcap_device.c`
- Public capture include: `include/capture.h`
- Placeholder packet include: `include/packet.h`
- No build system is currently present in the repository
- No GUI, API backend, UDP publisher, packet decoder, or storage module is
  currently implemented

`list_interface()` calls `pcap_findalldevs()`, prints each discovered interface
name, and releases the device list with `pcap_freealldevs()`.

## Project structure

```text
visualize_connect/
├── README.md
├── docs/
│   └── udp_backend_contract.md
├── include/
│   ├── capture.h
│   └── packet.h
└── src/
    └── capture/
        ├── README.md
        ├── capture.c
        └── pcap_device.c
```

## Planned architecture

The intended direction is:

```text
C capture process
  |
  | libpcap/Npcap
  v
Packet/event normalization
  |
  | UDP socket datagrams
  v
API backend
  |
  | REST/WebSocket/SSE API
  v
GUI client
```

Responsibilities:

- C capture process: enumerate interfaces, capture packets, decode enough
  metadata for visualization, and publish compact events over UDP.
- UDP socket boundary: provide a simple local transport from the C process to
  the backend without coupling the GUI directly to packet capture.
- API backend: receive UDP events, validate and aggregate them, store state if
  needed, and expose GUI-facing APIs.
- GUI: connect only to the API backend. The GUI should not read from libpcap or
  the UDP capture socket directly.

See `docs/udp_backend_contract.md` for the proposed UDP payload contract.

## Planned development milestones

1. Add a build system.
2. Declare implemented public functions in headers.
3. Add a small CLI entry point that calls `list_interface()`.
4. Add packet capture start/stop functions.
5. Decode Ethernet, IPv4, TCP, and UDP metadata.
6. Add a UDP socket publisher in C.
7. Add an API backend UDP receiver.
8. Add GUI views that consume backend APIs.

## Dependencies

Current code requires libpcap-compatible development headers:

- Linux/macOS: libpcap
- Windows: Npcap SDK and compatible compiler configuration

The planned GUI/backend stack is not implemented yet. Earlier notes mentioned
GTK, OpenGL, SDL, SQLite, and GeoIP, but those dependencies are not currently
used by the repository code.
