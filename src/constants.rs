pub const TARANTELLA_MM_TOML : &str = "# As Tarantella isn't 100% stable as of now, please refrain from editing any fields other than version
[package]
name = \"<app_name>\"
version = \"0.1.0\"
module_type = \"main_module\"
build_dir = \"<app_name>_latest\"
releases_repo = \"\"

[dependencies]";

pub const TARANTELLA_SM_TOML : &str = "# As Tarantella isn't 100% stable as of now, please refrain from editing any fields other than version
[package]
name = \"<app_name>\"
version = \"0.1.0\"
module_type = \"side_module\"
build_dir = \"<app_name>_latest\"
releases_repo = \"\"

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
BUILDDIR=build

$(P): $(OBJECTS)
\t$(EMCC) $(EMCC_CFLAGS) src/main.c -o $(BUILDDIR)/a.out.js";

pub const MAKEFILE_SM: &str = "P=<app_name>
OBJECTS=src/main.c
EMCC=emcc
EMCC_CFLAGS=-s SIDE_MODULE=2
BUILDDIR=<app_name>_latest

$(P): $(OBJECTS)
\t$(EMCC) $(EMCC_CFLAGS) src/main.c -o $(BUILDDIR)/$(P).wasm";

pub const INDEX_HTML: &str = "<html lang=\"en\">
  <head>
    <meta charset=\"UTF-8\" />
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />
    <title>Tarantella App</title>
  </head>
  <body>
    <h1>Hello, Tarantella 💃🕷</h1>
    <script async type=\"text/javascript\" src=\"build/a.out.js\"></script>
  </body>
</html>";

pub const GIT_IGNORE: &str = "<app_name>_latest/
releases/
dependencies/
";

pub const README: &str = "# Welcome to <app_name>! 💃🕷
This is an app made with the [Tarantella Package Manager](https://github.com/danbugs/tarantella)
";