#include "Ui.hpp"

Ui::Ui(sf::RenderWindow *window, Data *data, tool *Tool, std::map<uint8_t, sf::Color> *colors, Network *network) : _data(data), _window(window), _tool(Tool), _colors(colors), _network(network)
{
    _frame.setFillColor(sf::Color::Transparent);
    _frame.setOutlineColor(sf::Color::White);
    _frame.setOutlineThickness(2);
    _selected.setFillColor(sf::Color::Transparent);
    _selected.setOutlineColor(sf::Color::Black);
    _selected.setOutlineThickness(2);
    _hover.setFillColor(sf::Color(255, 255, 255, 100));
}

Ui::~Ui()
{
}

void Ui::draw()
{
    _window->setView( _window->getDefaultView());
    _window->draw(_frame);
    _window->setView(_window->getDefaultView());
    sf::Vector2f size = _window->mapPixelToCoords(sf::Vector2i(_window->getSize().x * 0.8, 60));
    sf::Vector2f origin = _window->mapPixelToCoords(sf::Vector2i(_window->getSize().x * 0.8, 60)) / 2.f;
    sf::Vector2f pos = _window->mapPixelToCoords(sf::Vector2i(_window->getSize().x / 2, _window->getSize().y - 60));

    _frame.setSize(size);
    _frame.setOrigin(origin);
    _frame.setOrigin(_frame.getOrigin());
    _frame.setPosition(pos);

    _selected.setSize(_window->mapPixelToCoords(sf::Vector2i(60, 60)));
    _selected.setOrigin(_window->mapPixelToCoords(sf::Vector2i(0, 30)));
    _hover.setSize(_window->mapPixelToCoords(sf::Vector2i(60, 60)));
    _hover.setOrigin(_window->mapPixelToCoords(sf::Vector2i(0, 30)));

    for (uint8_t i = 0; i < _colors->size() - 1; i++) {
        if (_toolIcon.size() <= i) {
            _toolIcon.push_back(sf::RectangleShape());
        }
        _toolIcon[i].setSize(_window->mapPixelToCoords(sf::Vector2i(60, 60)));
        _toolIcon[i].setOrigin(_window->mapPixelToCoords(sf::Vector2i(0, 30)));
        _toolIcon[i].setFillColor(_colors->at(i + 1));
        _toolIcon[i].setPosition(pos + sf::Vector2f(i * _toolIcon[i].getSize().x - origin.x, 0));
        _window->draw(_toolIcon[i]);
    }
    _window->draw(_hover);
    _window->draw(_selected);
}

void Ui::event(sf::Event *event)
{
    if (event->type == sf::Event::MouseButtonPressed) {
        if (event->mouseButton.button == sf::Mouse::Left) {
            sf::Vector2f mousePos = _window->mapPixelToCoords(sf::Vector2i(event->mouseButton.x, event->mouseButton.y));
            for (uint8_t i = 0; i < _colors->size() - 1; i++) {
                if (_toolIcon[i].getGlobalBounds().contains(mousePos)) {
                    _selected.setPosition(_toolIcon[i].getPosition());
                    _tool->color = i + 1;
                    return;
                }
            }
        }
    }
    if (event->type == sf::Event::MouseMoved) {
        sf::Vector2f mousePos = _window->mapPixelToCoords(sf::Vector2i(event->mouseMove.x, event->mouseMove.y));
        for (uint8_t i = 0; i < _colors->size() - 1; i++) {
            if (_toolIcon[i].getGlobalBounds().contains(mousePos)) {
                _hover.setPosition(_toolIcon[i].getPosition());
                return;
            }
        }
    }
}