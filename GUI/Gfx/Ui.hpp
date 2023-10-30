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
        Ui(sf::RenderWindow *window, Data *data, tool *Tool);
        ~Ui();
        void draw();

    protected:
    private:
        Data *_data;
        sf::RenderWindow *_window;
        sf::RectangleShape _hover;
        sf::RectangleShape _selected;
        sf::RectangleShape _background;
        sf::RectangleShape _frame;

        tool *_tool;
};
