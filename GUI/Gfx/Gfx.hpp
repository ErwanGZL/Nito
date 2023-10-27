#pragma once
#include <iostream>
#include <string>
#include <vector>
#include <thread>

#include <SFML/Graphics.hpp>
#include <SFML/Window.hpp>
#include <SFML/System.hpp>
#include "Data.hpp"
#include "Network.hpp"

void *threadNet(void *arg);

class Gfx {
    public:
        Gfx();
        ~Gfx();
        void run();
        sf::Color getColor(int value)
        {
            switch (value) {
                case 0:
                    return sf::Color::Black;
                case 1:
                    return sf::Color::White;
                case 2:
                    return sf::Color::Red;
                case 3:
                    return sf::Color::Green;
                case 4:
                    return sf::Color::Blue;
                case 5:
                    return sf::Color::Yellow;
                case 6:
                    return sf::Color::Magenta;
                case 7:
                    return sf::Color::Cyan;
                case 8:
                    return sf::Color::Transparent;
                default:
                    return sf::Color::Black;
            }
        }

    protected:
    private:
        std::thread _thread;
        Data *_data;
        sf::RenderWindow _window;
        sf::RectangleShape _rect;
        sf::Event _event;
};
