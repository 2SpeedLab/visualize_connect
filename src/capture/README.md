```
pcap_findalldevs(&devices, errbuf)
              |
              v
devices -> [Wi-Fi] -> [Ethernet] -> [VPN] -> NULL
              |
              v
       loop through each one
              |
              v
       print d->name
              |
              v
       free the entire list
```