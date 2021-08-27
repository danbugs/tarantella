# Tarantella 💃🕷

Tarantella is an **incoming** package manager for WASM modules meant to be dynamically linked!

## Usage

```
tapm 0.1.0

USAGE:
    tapm.exe <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add      Add a new dependency to your wasm app (e.g., `--add "dcw-0.1.0"`)
    build    Build your wasmp app
    help     Prints this message or the help of the given subcommand(s)
    new      Create a new wasm app (e.g., `--new "dancing_web" [-s]`)
```

## Roadmap

- [x] Check if users have `emcc` installed. If not, exit and display and error message telling them to install it w/ a URL.
- [x] Allow users to create a WASM main module app w/ `tapm new <app_name>`.

  - [x] Create `Tarantella.toml`.
  - [x] Populate `Tarantella.toml` w/ the following content:

    ```
    [package]
    name = "<app_name>"
    version = "0.1.0"
    type="main_module"

    [dependencies]
    ```

  - [x] Create a `src` folder, and a `main.c` file inside it w/ the following content:

    ```
    #include <stdio.h>
    #include <emscripten.h>

    EMSCRIPTEN_KEEPALIVE
    void print_hello() {
        printf("Hello!\n");
    }

    int main() {
        print_hello()
    }
    ```

  - [x] Create a `Makefile` w/ the following content:

    ```
    P=<app_name>
    OBJECTS=src/main.c
    EMCC=emcc
    EMCC_CFLAGS=-s MAIN_MODULE=1

    $(P): $(OBJECTS)
        $(EMCC) $(EMCC_CFLAGS) src/$(P).c
    ```

  - [x] Create a `dependencies` folder.

- [x] Allow users to create a WASM side module app w/ `tapm new <app_name> --side-module` (`--side-module` can be abbreviated `-s`).

  - [x] Create `Tarantella.toml`.
  - [x] Populate `Tarantella.toml` w/ the following content:

    ```
    [package]
    name = "<app_name>"
    version = "0.1.0"
    type="side_module"

    [dependencies]
    ```

  - [x] Create a `src` folder, and a `main.c` file inside it w/ the following content:

    ```
    #include <stdio.h>
    #include <emscripten.h>

    EMSCRIPTEN_KEEPALIVE
    void print_hello() {
        printf("Hello!\n");
    }

    int main() {
        print_hello()
    }
    ```

  - [x] Create a `<app_name>_latest` folder.
  - [x] Create a `releases` folder.
  - [x] Create a `Makefile` w/ the following content:

    ```
    P=<app_name>
    OBJECTS=src/main.c
    EMCC=emcc
    EMCC_CFLAGS=-s SIDE_MODULE=2
    BUILDDIR=<app_name>_latest

    $(P): $(OBJECTS)
        $(EMCC) $(EMCC_CFLAGS) src/$(P).c -o $(BUILDDIR)/$(P).wasm
    ```

  - [x] Create a `dependencies` folder.

- [ ] Allow users to run a main module w/ `tapm build` (i.e., a wrapper around `emmake make`).

  - [ ] If users run this from a directory where there isn't a `Makefile`, tell them to run it from the project's root folder.

- [ ] Allow users to create a Tarantella account w/ `tapm register`.

  - Prompt users to enter an e-mail.
  - Prompt users to enter an username.
  - Prompt users to enter a password (invisible).
  - Return error message if e-mail or username already exists.
  - After registration, tell users to verify their e-mail address and login in the CLI.

- [ ] Allow users to login to a Tarantella account w/ `tapm login`.
  - Prompt users to enter their username.
  - Prompt users to enter their password (invisible).
  - If users is not verified, display an error message tell them to verify their e-mail address.
  - On first login, create a folder namespacing their packages w/ their username.

> Note: In the future, allow users to request to re-send verificiation e-mail and allow users to request their username/e-mail/password.

- [ ] Allow users to publish a side module w/ `tapm publish`

  - If users run this from a directory where there isn't a `<app_name>_latest` folder, tell them to run it form the project's root folder.
  - Check for module type, if no type is specified, display an error message. If users run this from a project of `main_module` type, say that `main_modules` are publishable yet.
  - Zip `<app_name>_latest` and call it `<app_name>-<version>` (i.e., all obtained from `Tarantella.toml`).
  - Upload zipped folder to their namespace on the server.

- [ ] Allow users to add dependencies to their project w/ `tapm add <dependency_name>[-<version>]` (version is optional, if not included add latest).

  - If the dependency is not found, return an error.
  - Download zipped dependency to `dependencies` folder.
  - Unzip dependency in `dependencies` folder.
  - Delete zip of dependency.
  - Add dependency to `Tarantella.toml`:

  ```
    [dependencies]
    <dependency_name>=<version>
  ```
