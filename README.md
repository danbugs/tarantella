# Tarantella 💃🕷

Tarantella is an **incoming** package manager for WASM modules meant to be dynamically linked!

## Roadmap

- [ ] Check if users have `emcc` installed. If not, exit and display and error message telling them to install it w/ a URL.
- [ ] Allow users to create a WASM main module app w/ `tapm new <app_name> [--main-module or -mm]` (`--main-module` or `-mm` is default and optional).

  - Create `Tarantella.toml`.
  - Populate `Tarantella.toml` w/ the following content:

    ```
    [package]
    name = "<app_name>"
    version = "0.1.0"
    type="main_module"

    [dependencies]
    ```

  - Create a `src` folder, and a `main.c` file inside it w/ the following content:

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

  - Create a `Makefile` w/ the following content:

    ```
    P=<app_name>
    OBJECTS=src/main.c
    EMCC=emcc
    EMCC_CFLAGS=-s MAIN_MODULE=1

    $(P): $(OBJECTS)
        $(EMCC) $(EMCC_CFLAGS) src/$(P).c
    ```

  - Create a `packages` folder.

- [ ] Allow users to create a WASM side module app w/ `tapm new <app_name> --side-module` (`--side-module` can be abbreviated `-sm`).

  - Create git repository.
  - Create `Tarantella.toml`.
  - Populate `Tarantella.toml` w/ the following content:

    ```
    [package]
    name = "<app_name>"
    version = "0.1.0"
    type="side_module"

    [dependencies]
    ```

  - Create a `src` folder, and a `lib.c` file inside it w/ the following content:

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

  - Create a `<app_name>_latest` folder.
  - Create a `Makefile` w/ the following content:

    ```
    P=<app_name>
    OBJECTS=src/main.c
    EMCC=emcc
    EMCC_CFLAGS=-s SIDE_MODULE=2
    BUILDDIR=<app_name>_latest

    $(P): $(OBJECTS)
        $(EMCC) $(EMCC_CFLAGS) src/$(P).c -o $(BUILDDIR)/$(P).wasm
    ```

  - Create a `packages` folder.

- [ ] Allow users to run a main module w/ `tapm build` (i.e., a wrapper around `emmake make`).

  - If users run this from a `side_module` (i.e., identified from `type` field in `Tarantella.toml`), display a error message saying that side modules can't be run.
  - If `type` field is not present in `Tarantella.toml` file, say: "Is this a side module? If so, make sure to add `type="side_module"` to `Tarantella.toml` and try again"
  - If users run this from a directory where there isn't a `Makefile`, tell them to run it from the project's root folder.

- [ ] Allow users to create a Tarantella account w/ `tapm register`.

  - Prompt users to enter an e-mail.
  - Prompt users to enter an username.
  - Prompt users to enter a password (invisible).
  - Return error message if e-mail or username already exist.
  - After registration, tell users to verify their e-mail address and login in the CLI.

- [ ] Allow users to login to a Tarantella account w/ `tapm login`.
  - Prompt users to enter their username.
  - Prompt users to enter their password (invisible).
  - If users is not verified, display an error message tell them to verify their e-mail address.
  - On first login, create a folder namespacing their packages w/ username on the folder.

> Note: In the future, allow users to request to re-send verificiation e-mail and allow users to request their username/e-mail/password.

- [ ] Allow users to publish a side module w/ `tapm publish`

  - If users run this from a directory where there isn't a `<app_name>_latest` folder, tell them to run it form the project's root folder.
  - If users run this from a project of `main_module` type, say that `main_modules` are publishable yet.
  - Create a `releases` folder.
  - Zip `<app_name>_latest` and call it `<app_name>-<version>` (i.e., all obtained from `Tarantella.toml`).
  - Upload zipped folder to their namespace on the server.

- [ ] Allow users to add dependencies to their project w/ `tapm add <dependency_name>[-<version>]` (version is optional, if not included add latest).

  - If the dependency is not found, return an error.
  - Download zipped dependency to `packages` folder.
  - Unzip dependency in `packages` folder.
  - Delete zip of dependency.
  - Add dependency to `Tarantella.toml`:

  ```
    [dependencies]
    <dependency_name>=<version>
  ```


