#include "Gfx.hpp"

Gfx::Gfx()
{
    _data = new Data();
    _window.create(sf::VideoMode(800, 600), "Nito");
    _window.setFramerateLimit(60);
}

Gfx::~Gfx()
{
    _data->setRunning(false);
    _thread.join();
    delete _data;
}

void Gfx::run ()
{
    _data->setPort(4242);
    _data->setMachine("127.0.0.1");
    _thread = std::thread(threadNet, _data);
    while (_data->isRunning()) {
        _data->lock();
        while (_window.pollEvent(_event)) {
            if (_event.type == sf::Event::Closed)
                _data->setRunning(false);
        }
        int d = std::min(_window.getSize().x / _data->getWidth(), _window.getSize().y / _data->getHeight());
        _rect.setSize(sf::Vector2f(d, d));
        for (int i = 0; i < _data->getWidth(); i++) {
            for (int j = 0; j < _data->getHeight(); j++) {
                if (_data->getCell(i, j) == 0) continue;
                _rect.setPosition(i * d, j * d);
                _rect.setFillColor(getColor(_data->getCell(i, j)));
                _window.draw(_rect);
            }
        }
        _data->unLock();
    }
}

void *threadNet(void *arg)
{
    Data *data = (Data *)arg;
    try {
        Network network(data);
        network.run();
    } catch (std::exception &e) {
        std::cerr << e.what() << std::endl;
        data->setRunning(false);
    }
    return NULL;
}