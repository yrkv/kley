# Kley (Pre-Alpha!)

An elegant language for system automation and utilities.
Kley strives to make writing maintainable shell scripts convenient and easy.

> [!CAUTION]
> This language is currently in the **proof-of-concept** stage, and there will be many breaking changes.

## Contents

- [Overview](#overview)
- [Examples](#examples)
- [Other](#other)

## Overview

Kley is a shell scripting language, meaning it is designed specifically for
running commands and manipulating their inputs/outputs. Most commonly used examples
of shell scripting languages are simply shells, such as Bash or Powershell.

However, shell scripts are notorious for becoming unmaintainable and excessively
complicated as they grow. By not trying to also be a shell, Kley is able to be
significantly cleaner and have features which fit well in scripts but are
antithetical to a shell.

> [!NOTE]
> Kley is not designed to be a shell. Kley's philosophy is that good shells make
> poor programming languages, and good programming languages make poor shells.
> By embedding a simple shell into the language, it can have the best of both
> for scripts.

## Examples

TODO

## Other

### Current Project State
Kley is currently in pre-alpha with the bare minimum implemented to write a few
demo scripts. The primary focus right now is planning and filling out the core
features to start transitioning the scripts I use to Kley.

> [!CAUTION]
> In pre-alpha, anything may change at any time.

#### Core Features, TODO for v0.2
- [x] Lexer, Parser, Interpreter
- [ ] Basic types: `int`, `str`, `bool`, `list<T>`, `unit`
- [ ] Internal shell / command mode + types: `command`, `output`
- [ ] Functions
- [ ] Basic control flow: `if`, `while`

#### Planned before v1.0 release
- [ ] REPL
- [ ] Error handling (tightly integrated with command mode)
- [ ] Complete type system: `float`, `tuple`, `map<K, V>`, structs, enums
- [ ] JSON: builtin `json` type, with decoding and encoding
- [ ] Generics (or similar? no clear plan yet!)
- [ ] Module system / standardized way to cleanly split code into multiple files
- [ ] Kley scripts invoking other Kley scripts can call them as a function
- [ ] Transpiler to Bash (similar goal as [amber](https://amber-lang.com/))
- [ ] JIT Compiler (cached compilation for scripts which run often?)

### Alternatives
- [Hush](https://hush-shell.github.io/) has similar goals and execution, with
  the primary differences being a lua-like syntax and some different design
  choices regarding command blocks. Specifically, Hush makes the internal shell
  bash-like, while Kley keeps command mode minimal.
- [Amber](https://amber-lang.com/) is a type-safe modern language which is
  transpiled to Bash.
- [sh](https://sh.readthedocs.io/en/latest/index.html) or the lua version
  [luash](https://github.com/zserge/luash) provide clean shell integration into
  python or lua.
- [Nushell](https://www.nushell.sh/) and [Elvish](https://elv.sh/) both make a
  very strong case for a shell being able to be a solid programming language.

### Pronounciation/Meaning
"kley" is a transliteration of the Ukrainian word "клей", meaning "glue". It is
pronounced (almost) identically to the English word "clay".

