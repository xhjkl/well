# ✨♟️ well

A command-line utility to chat with your codebase.

## Installation

```
cargo install well
```

## Usage

```
$ echo OPENAI_API_KEY='...' > .env

$ well is it possible to link against this statically as a library\?

>> is it possible to link against this statically as a library?

<< f{"path": "."}
<< F{"path":"Cargo.toml"}
<< f{"path":"src"}

<< The `src` directory does not contain a `lib.rs` file,
   suggesting that the project does not explicitly define a library module.
   The presence of `main.rs` reinforces that it is a standalone binary.

   To link it statically, you would generally need to modify the project
   to include a `lib.rs` and configure `Cargo.toml` accordingly.
```

In the above dialog, the model shows which files were read, and then the model provides an answer.

This might send the current directory contents to OpenAI servers at the model's discretion,
but the model is not allowed to step outside the directory the program was run at.

## Naming

It's named so that the terminal invocation reads as natural language:
"well, what is the most complex function?.."

```
$ well what is the most complex function\?
```
