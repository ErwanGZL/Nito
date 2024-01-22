#include "Gfx.hpp"

Gfx::Gfx()
{
    _data = new Data();
    _data->setPort(4242);
    _data->setMachine("127.0.0.1");
    _network = new Network(_data);
    _window.create(sf::VideoMode(800, 600), "Nito");
    _window.setFramerateLimit(60);
    _tools = std::unique_ptr<Ui>(new Ui(&_window, _data, &_tool, &_colors, _network));
    _cellMap = std::unique_ptr<CellMap>(new CellMap(&_window, _data, &_tool, &_colors, _network));
    _tool.size = 5;
}

Gfx::~Gfx()
{
    _data->setRunning(false);
    _thread.join();
    delete _data;
}

void Gfx::run()
{
    _thread = std::thread(threadNet, _network);
    while (_data->isRunning())
    {
        _data->lock();
        while (_window.pollEvent(_event))
        {
            event();
            _cellMap->event(&_event);
        }
        draw();
        _cellMap->event();
        _data->unLock();
        usleep(10000);
    }
}

void Gfx::draw()
{
    _window.clear(sf::Color(255, 255, 240));
    _tools->draw();
    _cellMap->draw();
    _window.display();
}

void Gfx::event()
{
    if (_event.type == sf::Event::Closed)
    {
        _data->setRunning(false);
        _window.close();
    }
    _tools->event(&_event);
}

void *threadNet(void *arg)
{
    Network *network = (Network *)arg;
    try
    {
        network->run();
    }
    catch (std::exception &e)
    {
        std::cerr << e.what() << std::endl;
        // data->setRunning(false);
    }
    return NULL;
}