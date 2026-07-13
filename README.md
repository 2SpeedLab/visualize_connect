# App visualize network connect

## Framework/Library support for project:
- libpcap: support windows,mac,linux
- GUI expected: using gtk/opengl/sdl2

## Project structure
This section describes the main layout of the project and how the core pieces fit together.

### Technology stack
- Language: C
- Packet capture: libpcap / Npcap
- GUI: GTK4
- Visualization: SDL3
- GeoIP: MaxMind GeoLite2
- Storage: SQLite

### Project tree
```text
visualize_connect/
├── CMakeLists.txt
├── README.md
├── LICENSE
├── include/
│   ├── packet.h
│   ├── capture.h
│   ├── session.h
│   ├── stats.h
│   ├── geoip.h
│   ├── protocol.h
│   ├── database.h
│   ├── config.h
│   └── app.h
├── src/
│   ├── main.c
│   ├── capture/
│   │   ├── capture.c
│   │   ├── pcap_device.c
│   │   └── packet_decoder.c
│   ├── protocol/
│   │   ├── ethernet.c
│   │   ├── ipv4.c
│   │   ├── ipv6.c
│   │   ├── tcp.c
│   │   ├── udp.c
│   │   ├── icmp.c
│   │   ├── dns.c
│   │   ├── tls.c
│   │   └── http.c
│   ├── analysis/
│   │   ├── session.c
│   │   ├── statistics.c
│   │   ├── bandwidth.c
│   │   ├── alerts.c
│   │   └── ids.c
│   ├── geoip/
│   │   └── geoip.c
│   ├── storage/
│   │   ├── sqlite.c
│   │   ├── pcap_writer.c
│   │   ├── pcap_reader.c
│   │   └── export.c
│   ├── gui/
│   │   ├── gtk/
│   │   │   ├── window.c
│   │   │   ├── packet_list.c
│   │   │   ├── filters.c
│   │   │   ├── stats_panel.c
│   │   │   └── settings.c
│   │   └── sdl/
│   │       ├── renderer.c
│   │       ├── map.c
│   │       ├── animation.c
│   │       └── camera.c
│   ├── util/
│   │   ├── logger.c
│   │   ├── thread.c
│   │   ├── queue.c
│   │   ├── config.c
│   │   └── timer.c
│   └── core/
│       ├── app.c
│       ├── event_bus.c
│       └── worker.c
├── assets/
│   ├── world_map.png
│   ├── icons/
│   └── GeoLite2/
├── docs/
│   ├── architecture.md
│   └── roadmap.md
└── tests/
```

### Processing flow
```text
Capture Thread -> Packet Queue -> Packet Decoder -> Analysis Engine -> Event Queue
                                      │
                                      └── Session Manager
                                              │
                                              ├── Statistics
                                              ├── GeoIP
                                              ├── Alerts
                                              ├── IDS
                                              └── Storage
```

### UI flow
```text
GTK4 UI          SDL3 Renderer
   │                  │
   └────── Event Queue ──────┘
```

### Initial development goal
```text
Thread 1: Packet Capture
Thread 2: Packet Analysis
Thread 3: GTK UI
Thread 4: SDL Renderer
```

### Early milestone
- Phase 1: Start with a small and simple flow
- Main files: main.c, capture.c, packet_decoder.c, statistics.c, GTK packet list

### Features
- Enumerate interfaces
- Choose an interface
- Start capture
- Decode Ethernet
- Decode IPv4
- Decode TCP/UDP
- Display packet list