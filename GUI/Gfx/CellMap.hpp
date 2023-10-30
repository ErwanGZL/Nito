#pragma once
#include <iostream>
#include <string>
#include <vector>

#include <SFML/Graphics.hpp>
#include <SFML/Window.hpp>
#include <SFML/System.hpp>

#include "Data.hpp"
#include "tool.hpp"

class CellMap {
    public:
        CellMap(sf::RenderWindow *window, Data *data, tool *Tool);
        ~CellMap();
        void draw();
        sf::Color getColor(uint8_t value)
        {
            switch (value) {
                case 0:
                    return sf::Color::Transparent;
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
        Data *_data;
        sf::RenderWindow *_window;
        sf::RectangleShape _rect;
        sf::RectangleShape _frame;
        tool *_tool;
};
