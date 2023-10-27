cmake_minimum_required(VERSION 3.0)
project(nito)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake_modules" ${CMAKE_MODULE_PATH})
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11 -pthread -O3 -g3")

if (APPLE)
    include_directories(/opt/homebrew/Cellar/sfml/2.5.1_2)
    link_directories(/opt/homebrew/Cellar/sfml/2.5.1_2/lib)
    set(CMAKE_PREFIX_PATH /opt/homebrew/Cellar/sfml/2.5.1_2)
    find_library(LIBCONFIGPP libconfig++.dylib PATHS /opt/homebrew/Cellar/sfml/2.5.1_2/lib)
else()
    find_package(SFML 2.5 COMPONENTS system window graphics REQUIRED)
endif()
# Add source files to your project
set(
    SOURCES
    main.cpp
    Gfx/Gfx.cpp
    Net/Network.cpp
)

set(
    INCLUDES_DIR
    Net/
    Gfx/
    Data/
)

# Select the appropriate SFML libraries based on the system architecture
if(APPLE)
    if(CMAKE_OSX_ARCHITECTURES STREQUAL "arm64")
        set(SFML_LIBRARIES
            /opt/homebrew/Cellar/sfml/2.5.1_2/lib/libsfml-graphics.2.5.1.dylib
            /opt/homebrew/Cellar/sfml/2.5.1_2/lib/libsfml-window.2.5.1.dylib
            /opt/homebrew/Cellar/sfml/2.5.1_2/lib/libsfml-system.2.5.1.dylib
            /opt/homebrew/Cellar/sfml/2.5.1_2/lib/libsfml-audio.2.5.1.dylib
            /opt/homebrew/Cellar/sfml/2.5.1_2/lib/libsfml-network.2.5.1.dylib
        )
    endif()
else()
    set(SFML_LIBRARIES
        sfml-graphics
        sfml-window
        sfml-system
        sfml-audio
        sfml-network
    )
endif()

# Create an executable target
add_executable(${PROJECT_NAME} ${SOURCES})

message(STATUS "SFML_LIBRARIES: ${SFML_LIBRARIES}")
target_link_libraries(${PROJECT_NAME} ${SFML_LIBRARIES})
target_include_directories(${PROJECT_NAME} PUBLIC ${INCLUDES_DIR})
