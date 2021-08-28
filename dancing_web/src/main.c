#include <stdio.h>
#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE
void print_hello() {
    printf("Hello!\n");
}

int main() {
    print_hello();
}