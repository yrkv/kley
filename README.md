# Kley (Pre-Alpha!)

An elegant language for system automation and utilities. Kley strives to make writing maintainable shell scripts convenient and easy.

> [!CAUTION]
> This language is currently in the **proof-of-concept** stage, and there will be many breaking changes.

## Contents

- [Overview](#overview)
- [Examples](#examples)
- [Other](#other)

## Overview

Kley is a shell scripting language, designed specifically for running commands and manipulating their inputs/outputs. Most commonly used examples of shell scripting languages are simply shells such as Bash or Powershell, but those introduce heavy compromises when writing scripts.

Shell scripts are notorious for becoming unmaintainable and excessively complicated as they grow. By not trying to also be a shell and having shell scripts in mind from the start, Kley scripts is able to be significantly cleaner and have features which fit well in scripts but are antithetical to a shell.

> [!NOTE]
> Kley is not designed to be a shell. Kley's philosophy is that good shells make
> poor programming languages, and good programming languages make poor shells.
> By embedding a simple shell into the language, it can have the best of both
> for scripts.

## Examples

TODO

## Other

### Current Project State
Kley currently has the bare minimum implemented to write a few demo scripts, and nothing more. The primary focus right now is planning and filling out the core features to start transitioning the scripts I use to Kley.

For the time being, performance is intentionally being completely ignored in favor of developing a reliable and intuitive language.

> [!CAUTION]
> For now, anything may change at any time.

#### Core Features, TODO for v0.1 release
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

### Pronounciation/Meaning
"kley" is a transliteration of the Ukrainian word "клей", meaning "glue". It is
pronounced (almost) identically to the English word "clay".

### Alternatives
- [Hush](https://hush-shell.github.io/) has similar goals and execution, with the primary differences being a lua-like syntax and a more Bash-like internal shell.
- [Amber](https://amber-lang.com/) is a type-safe modern language which is transpiled to Bash.
- [Oils](https://www.oilshell.org/) focuses on improving the experience of writing shell scripts.
- [Koi](https://koi-lang.dev/) is a minimalistic shell scripting language which merges a simple language with common shell behaviors.
- [sh](https://sh.readthedocs.io/en/latest/index.html) or the lua version [luash](https://github.com/zserge/luash) provide clean shell integration into python or lua.
- [Nushell](https://www.nushell.sh/) and [Elvish](https://elv.sh/) both make a very strong case for a shell being able to be a solid programming language.


