#include "Gfx.hpp"

Gfx::Gfx()
{
    _data = new Data();
    _window.create(sf::VideoMode(800, 600), "Nito");
    _window.setFramerateLimit(60);
    _tools = std::unique_ptr<Ui>(new Ui(&_window, _data, &_tool, &_colors));
    _cellMap = std::unique_ptr<CellMap>(new CellMap(&_window, _data, &_tool, &_colors));

    _tool.shape = toolShape::CIRCLE;
    _tool.type = toolType::BRUSH;
    _tool.size = 5;
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
            event();
            _tools->event(&_event);
        }
        draw();
        _data->unLock();
        usleep(1000);
    }
}

void Gfx::draw()
{
    _window.clear();
    _tools->draw();
    _cellMap->draw();
    _window.display();
}

void Gfx::event()
{
    if (_event.type == sf::Event::Closed) {
        _data->setRunning(false);
        _window.close();
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