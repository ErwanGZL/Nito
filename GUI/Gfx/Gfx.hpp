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

class Gfx
{
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
        {1, sf::Color(33, 163, 219)},   // water
        {2, sf::Color(246, 215, 176)},  // sand
        {3, sf::Color(130, 94, 51)},    // wood
        {4, sf::Color(255, 85, 33)},    // fire
        {5, sf::Color(200, 200, 200)},  // smoke
        {6, sf::Color(0, 200, 0)},      // acid
        {7, sf::Color(237, 54, 33)},    // ember
        {8, sf::Color(200, 200, 0)},    // gas
        {9, sf::Color(80, 80, 80)},     // stone
        {10, sf::Color(30, 30, 30)},    // coal
        {11, sf::Color(255, 255, 255)}, // salt
        {12, sf::Color(128, 128, 128)}, // cinder
        {13, sf::Color(255, 0, 0)},     // lava
        {14, sf::Color(126, 0, 135)},   // oil
        {15, sf::Color(0, 255, 0)},     // moss
        {16, sf::Color(255, 255, 0)},   // canon powder
        {17, sf::Color(0, 255, 255)},   // ice
    };
};
