pub const TARANTELLA_MM_TOML : &str = "# WARNING: spacing is important in Tarantella.toml — keep all your fields w/ spaces around the equal-sign
[package]
name = \"<app_name>\"
version = \"0.1.0\" # format: major.minor.patch
module_type = \"main_module\" # main_module || side_module
build_dir = \"build\" # when modifying this field, change BUILD_DIR in the Makefile too
releases_repo = \"\" # format: https://github.com/<owner>/<repo_name>

[dependencies] # dependencies should not be added manually, use the tapm add command instead
";

pub const TARANTELLA_SM_TOML : &str = "# WARNING: spacing is important in Tarantella.toml — keep all your fields w/ spaces around the equal-sign
[package]
name = \"<app_name>\"
version = \"0.1.0\"  # format: major.minor.patch
module_type = \"side_module\" # main_module || side_module
build_dir = \"build\" # when modifying this field, change BUILD_DIR in the Makefile too
releases_repo = \"\" # format: https://github.com/<owner>/<repo_name>

[dependencies] # dependencies should not be added manually, use the tapm add command instead
";

pub const MAIN_C_MM : &str = "#include <stdio.h>
#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE
void print_hello() {
    printf(\"Hello!\\n\");
}

int main() {
    print_hello();
}";

pub const MAIN_C_SM : &str = "#include <stdio.h>
#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE
void print_hello() {
    printf(\"Hello!\\n\");
}";


pub const MAKEFILE_MM : &str = "P=<app_name>
OBJECTS=src/main.c
EMCC=emcc
EMCC_CFLAGS=-s MAIN_MODULE=1
BUILDDIR=build
DEPENDENCIES=

$(P): $(OBJECTS)
\t$(EMCC) $(EMCC_CFLAGS) $(DEPENDENCIES) src/main.c -o $(BUILDDIR)/$(P).js";

pub const MAKEFILE_SM: &str = "P=<app_name>
OBJECTS=src/main.c
EMCC=emcc
EMCC_CFLAGS=-s SIDE_MODULE=2 -c
BUILDDIR=build
DEPENDENCIES=

$(P): $(OBJECTS)
\t$(EMCC) $(EMCC_CFLAGS) $(DEPENDENCIES) src/main.c -o $(BUILDDIR)/$(P).o";

pub const INDEX_HTML: &str = "<html lang=\"en\">
  <head>
    <meta charset=\"UTF-8\" />
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />
    <title>Tarantella App</title>
  </head>
  <body>
    <h1>Hello, Tarantella 💃🕷</h1>
    <script async type=\"text/javascript\" src=\"build/<app_name>.js\"></script>
  </body>
</html>";

pub const GIT_IGNORE: &str = "build/
releases/
dependencies/
";

pub const README: &str = "# Welcome to <app_name>! 💃🕷
This is an app made with the [Tarantella Package Manager](https://github.com/danbugs/tarantella)

- Notes: 
  - This repository should always be in the same directory level as your source code's repository.
  - This repository should not be renamed.
";