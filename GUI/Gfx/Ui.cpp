#include "Ui.hpp"

Ui::Ui(sf::RenderWindow *window, Data *data, tool *Tool) : _window(window), _data(data), _tool(Tool)
{
    _frame.setFillColor(sf::Color::Transparent);
    _frame.setOutlineColor(sf::Color::White);
    _frame.setOutlineThickness(2);
}

Ui::~Ui()
{
}

void Ui::draw()
{
    _window->setView( _window->getDefaultView());
    _window->draw(_frame);
    _window->draw(_background);
    _window->draw(_hover);
    _window->draw(_selected);
    _window->setView(_window->getDefaultView());
    sf::Vector2f size = _window->mapPixelToCoords(sf::Vector2i(_window->getSize().x * 0.8, 60));
    sf::Vector2f origin = _window->mapPixelToCoords(sf::Vector2i(_window->getSize().x * 0.8, 60)) / 2.f;
    sf::Vector2f pos = _window->mapPixelToCoords(sf::Vector2i(_window->getSize().x / 2, _window->getSize().y - 60));

    _frame.setSize(size);
    _frame.setOrigin(origin);
    _frame.setOrigin(_frame.getOrigin());
    _frame.setPosition(pos);
}