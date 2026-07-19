#include "capture.h"
void list_interface(){
    char errbuf[PCAP_ERRBUF_SIZE];
    pcap_if_t * devices;
    if (pcap_findalldevs(&devices, errbuf) == -1){
        fprintf(stderr,"Error finding devices:%s\n",errbuf);
        return;
    }
    printf("Available interfaces\n");
    for (pcap_if_t *d = devices; d; d = d -> next)
    {
        printf("-%s\n",d->name);
    }
    pcap_freealldevs(devices);
}