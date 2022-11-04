#include <SDL.h>

#include <iostream>

#include "app.h"

int main(int argc, char const *argv[]) {
  App app;

  if (app.init().has_error()) return EXIT_FAILURE;

  while (app.step())
    ;
  app.dispose();

  return EXIT_SUCCESS;
}