#pragma once
#include <iostream>
#include <memory>
#include <map>

enum toolShape {
    CIRCLE,
    SQUARE,
    TRIANGLE,
};

enum toolType {
    ERASER,
    BRUSH,
};

struct tool
{
    toolShape shape;
    toolType type;
    uint8_t size;
} typedef tool;

