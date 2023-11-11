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
        Network *_network;

        tool _tool;
        std::map<uint8_t, sf::Color> _colors = {
            {0, sf::Color::Transparent},
            {1, sf::Color::White},
            {2, sf::Color::Red},
            {3, sf::Color::Green},
            {4, sf::Color::Blue},
            {5, sf::Color::Yellow},
            {6, sf::Color::Magenta},
            {7, sf::Color::Cyan}
        };
};
