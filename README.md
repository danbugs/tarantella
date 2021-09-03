# Tarantella Package Manager 💃🕷

Tarantella is a modern package manager for C/C++ WASM apps (main modules) and WASM libraries (side modules) meant to be dynamically linked!

## Requirements

- [basic-http-server](https://crates.io/crates/basic-http-server).
- [GitHub CLI](https://github.com/cli/cli#installation).
- [Emscripten](https://emscripten.org/docs/getting_started/downloads.html).

If you note weird behaviours with `tapm`, the version of one or more of the requirements might be outdated. I've created and used `tapm` with the following versions: ` emcc v2.0.29`, `basic-http-server v0.8.1`, and `gh v2.0.0`.

## Why use Tarantella?

Tarantella makes it easy to start, build, test, and distribute WASM apps and libraries by serving as a wrapper around several fantastic tools.

To start your app, run `tapm new "<app_name>" [-s]` (the `-s` makes it a side module). This automatically creates a C (but easily convertible to C++) WASM app with:
- an empty git repo,
- a dependencies folder for all side modules meant to be dynamically linked,
- a releases folder for all your upcoming releases,
- a src folder with a starting `main.c` file,
- a `.gitignore`,
- a `Makefile` to ease compilation, and
- a `Tarantella.toml` file that contains all of your project's pertinent info.

**To build your app**, run `tapm build`. This will use the created `Makefile` to create a new release in your build directory specified in the `Tarantella.toml` and `Makefile`. 

WASM main modules initialize with an `index.html` file. **To test your main module on the browser**, run `tapm run [-p <some_port_>]` — this will start a [`basic-http-server`](https://crates.io/crates/basic-http-server).

For publishing apps, Tarantella depends on GitHub. There are two options:
- If your repository already has a public remote origin hosted on GitHub: Tarantella will simply publish your releases there.
- If your repository does not have a remote origin, or has a private remote origin, or has a remote origin not hosted on GitHub: Tarantella will create a GitHub repository called `<app_name>_releases` and publish your releases there. 

**To publish your app**, run `tapm login` to login to GitHub, and `tapm publish` to create a new release. If you do not have a GitHub account, run: `tapm register` for more info on how to register for GitHub.

**To add dependencies to your app**, run `tapm add <owner>/<dependency_name> [<version>]` and any Tarantella libraries (i.e., published with `tapm publish`) will be automatically downloaded and added to a list of dependencies that are to be dynamically linked at compilation-time in your `Makefile`.

## Why not use [WAPM](https://wapm.io/)?

As of now, [WAPM](https://wapm.io/) only allows distributing `.wasm` modules. If your WASM app includes something like a JS library that goes alongside it, or you've compiled your dynamic library to a `.o`  file, you are left having to distribute that separetly — that sort of thing can really add complexity to, for example, setting up a WASM app that has several dynamically linked libraries.

## Install

```
cargo install tarantella
```

## Usage

```
tapm 0.6.8
tapm is a modern package manager for C/C++ WASM apps.

USAGE:
    tapm <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add         Add a new dependency to your wasm app (e.g., tapm add "danbugs/dancing_web" ["0.2.0"])
    build       Build your wasm app
    help        Prints this message or the help of the given subcommand(s)
    login       Login to GitHub to publish your wasm app with Tarantella
    new         Create a new wasm app (e.g., tapm new "dancing_web" 
[-s])
    publish     Publish a new release of your wasm app to GitHub with Tarantella (if your code is private, your
                release will be published to a separate repo)       
    register    Register to GitHub to publish your wasm app with Tarantella
    run         Start an HTTP server to test your main module (e.g., tapm run [-p 8000])
```

For more info on subcommands, run: `tapm <subcommand> --help`.

## Roadmap

Check out Tarantella's GitHub project board for a view of implemented subcommands [here](https://github.com/danbugs/tarantella/projects/1).

## Disclaimer

This package manager is new and definitely not free of bugs. However, this is a project I care about so I'll do my best to fix any issues you encounter while using `tapm`. To file a bug, see [this](https://github.com/danbugs/tarantella/issues).