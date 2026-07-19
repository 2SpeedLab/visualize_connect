# UDP backend contract

This document describes the planned transport boundary between the C capture
process and the API backend.

Status: design documentation only. The UDP publisher, API backend receiver, and
GUI API are not implemented in the current repository.

## Purpose

Use UDP as a lightweight local event stream from the C packet-capture process to
an API backend. The backend then exposes stable GUI-facing APIs.

The GUI should not connect directly to the C process or to libpcap. Keeping the
GUI behind the backend makes validation, aggregation, persistence, filtering,
and authentication easier to add later.

## Data flow

```text
C capture process
  |
  | UDP datagrams
  v
API backend UDP receiver
  |
  | internal event queue/storage
  v
REST/WebSocket/SSE API
  |
  v
GUI
```

## C socket requirements

The C process should create a UDP socket and send one event per datagram.

Recommended defaults for local development:

```text
Backend host: 127.0.0.1
Backend UDP port: 9000
Encoding: UTF-8 JSON for early development
Maximum datagram size: keep below typical MTU, preferably under 1200 bytes
```

Production builds can switch to a compact binary encoding after the event schema
is stable. JSON is easier to inspect while the protocol is still changing.

## Event envelope

Each datagram should contain one packet event:

```json
{
  "schema": "visualize_connect.packet.v1",
  "event": "packet",
  "timestamp_unix_ms": 0,
  "interface": "eth0",
  "direction": "unknown",
  "l3": "ipv4",
  "l4": "udp",
  "src_ip": "192.0.2.10",
  "src_port": 5353,
  "dst_ip": "198.51.100.20",
  "dst_port": 53,
  "bytes": 128
}
```

Field notes:

- `schema`: versioned schema name so the backend can reject incompatible events.
- `timestamp_unix_ms`: capture timestamp in Unix milliseconds.
- `interface`: capture interface name from libpcap.
- `direction`: `inbound`, `outbound`, or `unknown` until direction detection is
  implemented.
- `l3`: `ipv4`, `ipv6`, `arp`, or another decoded network-layer protocol.
- `l4`: `tcp`, `udp`, `icmp`, or another decoded transport-layer protocol.
- `src_port` and `dst_port`: optional when the packet has no transport ports.
- `bytes`: captured packet length or original wire length, but the chosen meaning
  must remain consistent.

## Backend responsibilities

The API backend should:

- Bind the UDP socket.
- Validate event schema and required fields.
- Drop malformed events without crashing the receiver.
- Aggregate packet events for GUI views.
- Expose GUI-facing APIs such as recent packets, bandwidth, protocol counts, and
  connection/session summaries.

The backend should treat UDP input as untrusted data, even on localhost.

## GUI responsibilities

The GUI should:

- Connect to the API backend, not the UDP capture socket.
- Request current state through REST or another request/response API.
- Subscribe to live updates through WebSocket or server-sent events if real-time
  updates are needed.
- Render backend-normalized data instead of decoding raw packets.

## Reliability notes

UDP does not guarantee delivery, ordering, or duplication protection. That is
acceptable for live visualization if occasional packet-event loss is tolerated.

If exact packet accounting becomes required, use a reliable transport or add
sequence numbers, acknowledgements, and replay support.
