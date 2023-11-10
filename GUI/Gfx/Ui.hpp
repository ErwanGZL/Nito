#pragma once
#include <iostream>
#include <string>
#include <vector>

#include <SFML/Graphics.hpp>
#include <SFML/Window.hpp>
#include <SFML/System.hpp>

#include "Data.hpp"
#include "tool.hpp"

class Ui {
    public:
        Ui(sf::RenderWindow *window, Data *data, tool *Tool, std::map<uint8_t, sf::Color> *colors);
        ~Ui();
        void draw();
        void event(sf::Event *event);

    protected:
    private:
        Data *_data;
        sf::RenderWindow *_window;
        sf::RectangleShape _hover;
        sf::RectangleShape _selected;
        sf::RectangleShape _frame;
        std::vector<sf::RectangleShape> _toolIcon;

        tool *_tool;
        std::map<uint8_t, sf::Color> *_colors;
};
