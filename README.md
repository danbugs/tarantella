# Tarantella 💃🕷

Tarantella is an **incoming** package manager for WASM modules meant to be dynamically linked!

## Install

```
cargo install tarantella
```

## Usage

```
tapm 0.4.0

USAGE:
    tapm <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build    Build your wasm app
    help     Prints this message or the help of the given subcommand(s)
    new      Create a new wasm app (e.g., `--new "dancing_web" [-s] 
`)
    run      Start an HTTP server to serve your main module
```

```
tapm-run 0.4.0
Start an HTTP server to serve your main module

USAGE:
    tapm run [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>    Specify a port [default: 4000]
```

```
tapm-new 0.4.0
Create a new wasm app (e.g., `--new "dancing_web" [-s] `)

USAGE:
    tapm new [FLAGS] <app-name>

FLAGS:
    -h, --help           Prints help information
    -s, --side-module    Create a side module
    -V, --version        Prints version information

ARGS:
    <app-name>    Name your new app
```

## Roadmap

Check out Tarantella's GitHub project board for a view of implemented subcommands [here](https://github.com/danbugs/tarantella/projects/1).