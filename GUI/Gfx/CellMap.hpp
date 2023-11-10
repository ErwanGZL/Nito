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
        CellMap(sf::RenderWindow *window, Data *data, tool *Tool, std::map<uint8_t, sf::Color> *colors);
        ~CellMap();
        void draw();

    protected:
    private:
        Data *_data;
        sf::RenderWindow *_window;
        sf::RectangleShape _rect;
        sf::RectangleShape _frame;
        tool *_tool;

        std::map<uint8_t, sf::Color> *_colors;
};
