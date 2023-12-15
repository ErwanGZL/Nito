#include "Network.hpp"

Network::Network(Data *data)
{
    _data = data;
    _socket = socket(PF_INET, SOCK_STREAM, 0);
    if (_socket == -1)
    {
        perror("socket");
        throw std::exception();
    }
    _addr.sin_family = AF_INET;
    _addr.sin_port = htons(_data->getPort());
    _addr.sin_addr.s_addr = inet_addr(_data->getMachine().c_str());
    if (::connect(_socket, (struct sockaddr *)&_addr, sizeof(_addr)) == -1)
    {
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
    int retval = select(_socket + 1, &_readfds, NULL, NULL, NULL);
    if (retval == -1 || retval == 0)
    {
        return -1;
    }

    std::memset(&_header, 0, sizeof(_header));
    ssize_t size = read(_socket, &_header, sizeof(_header));
    if (size == -1 || size == 0)
    {
        perror("read");
        return -1;
    }

    if (_header.size == 0)
        return 0;
    std::cout << "Received " << _header.size << " cells" << std::endl;

    std::memset(_buffer, 0, sizeof(_buffer));
    size = read(_socket, _buffer, _header.size * 5);
    while (size != _header.size * 5)
    {
        size += read(_socket, _buffer + size, _header.size * 5 - size);
    }
    if (size == -1 || size == 0)
    {
        perror("read");
        return -1;
    }
    return 0;
}

void Network::run()
{
    while (1)
    {
        if (handleMessages())
            return;
    }
}

int Network::handleMessages()
{
    if (getMessage() == -1)
        return 1;
    _data->lock();
    if (_data->isRunning() == false)
    {
        _data->unLock();
        return 1;
    }
    _data->setWidthHeight(_header.x, _header.y);
    for (int i = 0; i < _header.size; i++)
    {
        uint16_t x = _buffer[i * 5 + 1] << 8 | _buffer[i * 5];
        uint16_t y = _buffer[i * 5 + 3] << 8 | _buffer[i * 5 + 2];
        uint8_t value = _buffer[i * 5 + 4];
        _data->setCell(x, y, value);
    }
    _data->unLock();
    return 0;
}

void Network::sendCells(std::vector<cell> cells)
{
    uint16_t size = cells.size();
    if (size == 0)
        return;
    send(_socket, &size, sizeof(uint16_t), 0);
    std::vector<uint8_t> buffer(size * 5);
    for (int i = 0; i < size; i++)
    {
        buffer[i * 5] = cells[i].x & 0xFF;
        buffer[i * 5 + 1] = (cells[i].x >> 8) & 0xFF;
        buffer[i * 5 + 2] = cells[i].y & 0xFF;
        buffer[i * 5 + 3] = (cells[i].y >> 8) & 0xFF;
        buffer[i * 5 + 4] = cells[i].value;
    }
    send(_socket, buffer.data(), size * 5, 0);
}
