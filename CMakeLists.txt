cmake_minimum_required(VERSION 3.23.2)
set (CMAKE_CXX_STANDARD 20)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake/sdl2)

project(GraveyardShift VERSION 0.1.0)

file(GLOB SOURCES "src/*.cc")
add_executable(${PROJECT_NAME} ${SOURCES})

target_include_directories(${PROJECT_NAME} PRIVATE include)

# Add SDL2 library
find_package(SDL2 REQUIRED)
target_include_directories(${PROJECT_NAME} PRIVATE ${SDL2_INCLUDE_DIR})
target_link_libraries(${PROJECT_NAME} ${SDL2_LIBRARY})

# Add SDL2_image library
find_package(SDL2_image REQUIRED)
target_include_directories(${PROJECT_NAME} PRIVATE ${SDL2_IMAGE_INCLUDE_DIR})
target_link_libraries(${PROJECT_NAME} ${SDL2_IMAGE_LIBRARY})

# Add SDL2_ttf library
# find_package(SDL2_ttf REQUIRED)
# target_include_directories(${PROJECT_NAME} PRIVATE ${SDL2_TTF_INCLUDE_DIR})
# target_link_libraries(${PROJECT_NAME} ${SDL2_TTF_LIBRARY})

# Add SDL2_net library
# find_package(SDL2_net REQUIRED)
# target_include_directories(${PROJECT_NAME} PRIVATE ${SDL2_NET_INCLUDE_DIR})
# target_link_libraries(${PROJECT_NAME} ${SDL2_NET_LIBRARY})

# Add SDL2_mixer library
#find_package(SDL2_mixer REQUIRED)
# target_include_directories(${PROJECT_NAME} PRIVATE ${SDL2_MIXER_INCLUDE_DIR})
# target_link_libraries(${PROJECT_NAME} ${SDL2_MIXER_LIBRARY})

# Add SDL2_gfx library
# target_include_directories(${PROJECT_NAME} PRIVATE ${SDL2_GFX_INCLUDE_DIR})
# target_link_libraries(${PROJECT_NAME} ${SDL2_GFX_LIBRARY})

add_subdirectory("external/result")
target_link_libraries(${PROJECT_NAME} Result::Result)
