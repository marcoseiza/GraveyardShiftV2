#include "app.h"

#include <SDL.h>
#include <SDL_image.h>

#include "result.hpp"

App::App() {}
App::~App() {}

auto App::init() -> cpp::result<void, InitError> {
  window_width_ = 1280;
  window_height_ = 720;
  window_title_ = "Graveyard Shift";

  if (SDL_Init(SDL_INIT_VIDEO) < 0)
    return cpp::fail(InitError::VIDEO_NOT_SUPPORTED);

  uint32_t window_flags = SDL_WINDOW_SHOWN | SDL_WINDOW_RESIZABLE;
  window_ = SDL_CreateWindow(window_title_.c_str(), SDL_WINDOWPOS_UNDEFINED,
                             SDL_WINDOWPOS_UNDEFINED, window_width_,
                             window_height_, window_flags);

  if (window_ == nullptr) return cpp::fail(InitError::WINDOW_NOT_CREATED);

  uint32_t renderer_flags =
      SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC;
  renderer_ = SDL_CreateRenderer(window_, -1, renderer_flags);

  if (renderer_ == nullptr) return cpp::fail(InitError::RENDERER_NOT_CREATED);

  draw_rect_.w = 40;
  draw_rect_.h = 40;
  draw_rect_.x = 0;
  draw_rect_.y = 0;

  return {};
}

bool App::step() {
  SDL_Event event;

  draw_rect_.x += 20;

  draw();
  return true;
}

void App::dispose() {
  SDL_DestroyWindow(window_);
  SDL_DestroyRenderer(renderer_);
  SDL_Quit();
}

void App::draw() {
  SDL_SetRenderDrawColor(renderer_, 0, 0, 0, SDL_ALPHA_OPAQUE);
  SDL_RenderClear(renderer_);

  SDL_SetRenderDrawColor(renderer_, 255, 105, 180, SDL_ALPHA_OPAQUE);
  SDL_RenderFillRect(renderer_, &draw_rect_);
  SDL_RenderPresent(renderer_);
}