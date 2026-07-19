# Capture module

This directory contains the current packet-capture work.

## Implemented behavior

`pcap_device.c` defines:

```c
void list_interface(void);
```

The function:

1. Creates a libpcap error buffer.
2. Calls `pcap_findalldevs(&devices, errbuf)`.
3. Prints each discovered interface name.
4. Frees the device list with `pcap_freealldevs(devices)`.

Flow:

```text
pcap_findalldevs(&devices, errbuf)
              |
              v
devices -> [Wi-Fi] -> [Ethernet] -> [VPN] -> NULL
              |
              v
       loop through each pcap_if_t node
              |
              v
       print d->name
              |
              v
       pcap_freealldevs(devices)
```

## Planned UDP socket publisher

The capture module should eventually publish decoded packet events to the API
backend over UDP:

```text
libpcap capture -> packet decoder -> event serializer -> UDP socket -> backend
```

The capture process should only send compact packet metadata needed by the GUI.
The GUI should connect to the API backend, not directly to this capture module.

See `../../docs/udp_backend_contract.md` for the proposed event format and
socket boundary.
