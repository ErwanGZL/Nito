#pragma once
#include <iostream>
#include <string>
#include <sys/types.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/ip.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <fcntl.h>
#include <pthread.h>
#include <vector>
#include <math.h>
#include "Data.hpp"

struct header
{
    uint16_t x;
    uint16_t y;
    uint32_t size;
};

struct cell
{
    uint16_t x;
    uint16_t y;
    uint8_t value;
};

class Network {
    public:
        Network(Data *data);
        ~Network();
        void run();
        int getMessage();
        int handleMessages();
        void sendCells(std::vector<cell> cells);

    private:
        int _socket;
        struct sockaddr_in _addr;

        Data *_data;

        header _header;
        std::vector<cell> _buffer = std::vector<cell>(10000);
        fd_set _readfds;
        struct timeval _tv;
};
