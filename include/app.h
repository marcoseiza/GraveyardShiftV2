#ifndef __APP__H__
#define __APP__H__

#include <system_error>

#include "SDL_rect.h"
#include "result.hpp"

struct SDL_Window;
struct SDL_Renderer;

class App {
 public:
  App();
  ~App();

  enum class InitError {
    VIDEO_NOT_SUPPORTED,
    WINDOW_NOT_CREATED,
    RENDERER_NOT_CREATED
  };

  auto init() -> cpp::result<void, InitError>;
  bool step();
  void dispose();

 private:
  void draw();

  SDL_Rect draw_rect_;

  int window_width_;
  int window_height_;
  std::string window_title_;
  SDL_Window* window_ = nullptr;
  SDL_Renderer* renderer_ = nullptr;
};

#endif  //!__APP__H__