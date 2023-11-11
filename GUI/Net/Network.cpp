#include "Network.hpp"

Network::Network(Data *data)
{
    _data = data;
    _socket = socket(PF_INET, SOCK_STREAM, 0);
    if (_socket == -1) {
        perror("socket");
        throw std::exception();
    }
    _addr.sin_family = AF_INET;
    _addr.sin_port = htons(_data->getPort());
    _addr.sin_addr.s_addr = inet_addr(_data->getMachine().c_str());
    if (::connect(_socket, (struct sockaddr *)&_addr, sizeof(_addr)) == -1) {
        perror("connect");
        throw std::exception();
    }
    std::cout << "Connected to " << _data->getMachine() << ":" << _data->getPort() << std::endl;
}

Network::~Network()
{
}

int Network::getMessage()
{
    FD_ZERO(&_readfds);
    FD_SET(_socket, &_readfds);
    _tv.tv_sec = 1;
    _tv.tv_usec = 0;
    int retval = select(_socket + 1, &_readfds, NULL, NULL, &_tv);
    if (retval == -1 || retval == 0) {
        return -1;
    }
    size_t size = read(_socket, &_header, sizeof(_header));
    if (size == 0) return -1;
    size = read(_socket, _buffer.data(), _header.size * sizeof(cell));
    if (size == 0) return -1;
    return 0;
}

void Network::run()
{
    while (1) {
        if (handleMessages()) return;
    }
}

int Network::handleMessages()
{
    if (getMessage() == -1)
        return 1;
    _data->lock();
    if (_data->isRunning() == false) {
        _data->unLock();
        return 1;
    }
    _data->wipe();
    _data->setWidthHeight(_header.x, _header.y);
    for (int i = 0; i < _header.size; i++) {
        _data->setCell(_buffer[i].x, _buffer[i].y, _buffer[i].value);
    }
    _data->unLock();
    return 0;
}

void Network::sendCell(uint16_t x, uint16_t y, uint8_t value)
{
    cell cell = {x, y, value};
    send(_socket, &cell, sizeof(cell), 0);
}
