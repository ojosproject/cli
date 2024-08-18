# Ojos CLI Contributing Guidelines

![Ojos Project header](https://ojosproject.org/images/header.png)

## Table of Contents

- [Ojos CLI Contributing Guidelines](#ojos-cli-contributing-guidelines)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Setting up your development environment](#setting-up-your-development-environment)
    - [Branching Rules](#branching-rules)
    - [Tools](#tools)
    - [Development](#development)
    - [Installing](#installing)
    - [Publishing to `crates.io`](#publishing-to-cratesio)
  - [Adding a new subcommand](#adding-a-new-subcommand)
    - [`feature/mod.rs`](#featuremodrs)
    - [`feature/args.rs`](#featureargsrs)
    - [`feature/utils.rs`](#featureutilsrs)
    - [`src/args.rs`](#srcargsrs)
    - [`src/main.rs`](#srcmainrs)

## Introduction

Welcome to the contributing guide for the Ojos CLI! This will help you get
through contributing a feature.

## Setting up your development environment

### Branching Rules

Please fork the repository (or branch it!) and open a PR pointed towards the
`main` branch.

### Tools

This program primarily uses [clap](https://docs.rs/clap/latest/clap/), a Rust
crate for developing a CLI. There's more tools, but that depends on what
feature you're developing. For example, frontend features don't need anything
outside the standard library, but the newsletter features require
[reqwest](https://docs.rs/reqwest/latest/reqwest/).

Add tools as needed!

### Development

To run the CLI, commands that would start with `ojos` would have to be replaced
with `cargo run --`. For example, if I wanted to run the development version of
`ojos newsletter config --show`, I'd need to run:

```shell
cargo run -- newsletter config --show
```

If you'd like to see the available commands, you can run:

```shell
cargo run -- -h # dev env for ojos -h
```

### Installing

To install this program locally, you can run:

```shell
cargo install --path .
```

This would replace whatever version you have installed.

### Publishing to `crates.io`

Only Carlos can do this, but in case I forget:

```shell
cargo publish
```

## Adding a new subcommand

If you want to build a new subcommand, please create a new folder inside of the
`src` folder. Each folder is for a subcommand, such as `frontend` and
`newsletter`. Inside of each folder, there are three files. `args.rs`,
`utils.rs`, and `mod.rs`. They're the bare minimum, though you can create more
if needed.

### `feature/mod.rs`

The `mod.rs` file is only around to let `main.rs` know the other files inside of
your feature folder exists. For example, the `mod.rs` file inside of the
`newsletter` folder is:

```shell
pub mod utils;
pub mod args;
```

... just to show the existence of the other two files. If you create more files
inside of your feature folder, you should include it in `mod.rs`. You should
also add `mod [foldername]` to the top of the `main.rs` file.

### `feature/args.rs`

The `args.rs` file is where you can create CLI subcommands and options using
clap.
[This video helped me figure this out quickly](https://www.youtube.com/watch?v=fD9ptABVQbI).
I can't really explain it better than that video.

### `feature/utils.rs`

`utils.rs` can be considered the "`main.rs`" file of your feature folder. I
made a bunch of functions and threw them in there for the various different
things I need to implement the feature.

### `src/args.rs`

Now, in the `args.rs` file inside of `src`, you should add the structure you
created in `feature/args.rs`. Again, watch the video above to figure out how to
add it.

### `src/main.rs`

Finally, you can add your feature to the `main.rs` file by using `match`
statements. It's fairly easy to figure out, as you just need to match the
category, then the subcommand, and finally implement your resources in
`feature/utils.rs`.
