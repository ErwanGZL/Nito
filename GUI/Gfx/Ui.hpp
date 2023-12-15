#pragma once
#include <iostream>
#include <string>
#include <vector>

#include <SFML/Graphics.hpp>
#include <SFML/Window.hpp>
#include <SFML/System.hpp>

#include "Data.hpp"
#include "tool.hpp"
#include "Network.hpp"

class Ui
{
public:
    Ui(sf::RenderWindow *window, Data *data, tool *Tool, std::map<uint8_t, sf::Color> *colors, Network *network);
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
    sf::Text _textHover;
    sf::Font _font;
    std::vector<sf::RectangleShape> _toolIcon;
    Network *_network;
    tool *_tool;
    std::map<uint8_t, sf::Color> *_colors;
    std::vector<std::string> _toolName = {
        "Water",
        "Sand",
        "Wood",
        "Fire",
        "Smoke",
        "Acid",
        "Ember",
        "Gas",
        "Stone",
        "Coal",
        "Salt",
        "Cinder",
        "Lava",
        "Oil",
        "Moss",
        "Canon Powder",
        "Ice",
    };
};
