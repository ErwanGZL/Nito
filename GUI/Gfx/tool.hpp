#pragma once
#include <iostream>
#include <memory>

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

