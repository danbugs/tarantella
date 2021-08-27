pub const TARANTELLA_MM_TOML : &str = "[package]
name = \"<app_name>\"
version = \"0.1.0\"
type=\"main_module\"

[dependencies]";

pub const TARANTELLA_SM_TOML : &str = "[package]
name = \"<app_name>\"
version = \"0.1.0\"
type=\"side_module\"

[dependencies]";

pub const MAIN_C : &str = "#include <stdio.h>
#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE
void print_hello() {
    printf(\"Hello!\\n\");
}

int main() {
    print_hello();
}";

pub const MAKEFILE_MM : &str = "P=<app_name>
OBJECTS=src/main.c
EMCC=emcc
EMCC_CFLAGS=-s MAIN_MODULE=1

$(P): $(OBJECTS)
\t$(EMCC) $(EMCC_CFLAGS) src/main.c
";

pub const MAKEFILE_SM: &str = "P=<app_name>
OBJECTS=src/main.c
EMCC=emcc
EMCC_CFLAGS=-s SIDE_MODULE=2
BUILDDIR=<app_name>_latest

$(P): $(OBJECTS)
\t$(EMCC) $(EMCC_CFLAGS) src/main.c -o $(BUILDDIR)/$(P).wasm
";