#pragma once
#include <iostream>
#include <string>
#include <vector>
#include <thread>
#include <memory>

#include <SFML/Graphics.hpp>
#include <SFML/Window.hpp>
#include <SFML/System.hpp>
#include "Data.hpp"
#include "Network.hpp"
#include "Ui.hpp"
#include "CellMap.hpp"
#include "tool.hpp"

void *threadNet(void *arg);

class Gfx {
    public:
        Gfx();
        ~Gfx();
        void run();
        void event();
        void draw();

    protected:
    private:
        std::thread _thread;
        Data *_data;
        sf::RenderWindow _window;
        sf::Event _event;
        std::unique_ptr<Ui> _tools;
        std::unique_ptr<CellMap> _cellMap;

        tool _tool;
};
