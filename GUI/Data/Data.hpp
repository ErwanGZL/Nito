#pragma once
#include <mutex>
#include <vector>
#include <string>

class Data {
    public:
        Data() = default;
        ~Data() = default;
        void lock() { _mutex.lock();}
        void unLock() { _mutex.unlock();}
        void setCell(uint16_t x, uint16_t y, uint8_t value) { _map[x][y] = value;}
        void setPort(int port) { _port = port;}
        void setMachine(std::string machine) { _machine = machine;}
        int getPort() { return _port;}
        std::string getMachine() { return _machine;}
        bool isRunning() { return _isRunning;}
        void setRunning(bool isRunning) { _isRunning = isRunning;}
        std::vector<std::vector<uint8_t>> getMap() { return _map;}
        uint8_t getCell(uint16_t x, uint16_t y) { return _map[x][y];}
        uint16_t getWidth() { return _width;}
        uint16_t getHeight() { return _height;}

        void setWidthHeight(uint16_t width, uint16_t height) {
            if (_width != width) {
                _width = width;
                _height = height;
                _map = std::vector<std::vector<uint8_t>>(width, std::vector<uint8_t>(height, 0));
            } else if (_height != height) {
                _width = width;
                _height = height;
                _map = std::vector<std::vector<uint8_t>>(width, std::vector<uint8_t>(height, 0));
            }
        }

        void wipe() {
            for (int i = 0; i < _width; i++)
                for (int j = 0; j < _height; j++)
                    _map[i][j] = 0;
        }

    protected:
    private:
        std::mutex _mutex;
        uint16_t _width = 100;
        uint16_t _height = 100;
        int _port = 4242;
        std::string _machine = "127.0.0.1";
        std::vector<std::vector<uint8_t>> _map = std::vector<std::vector<uint8_t>>(_width, std::vector<uint8_t>(_height, 0));
        bool _isRunning = true;
};
