# ✨♟️ well

A command-line utility to chat with your codebase.

## Installation

```
cargo install well
```

## Usage

```
$ echo OPENAI_SECRET='...' > .env

$ well what is the most convoluted function across this codebase\?

>> what is the most convoluted function across this codebase?

<< f{
  "path": "./src"
}

<< F{
  "path": "./src/main.rs"
}

<< The most convoluted function in the codebase is the `run` function in the `main.rs` file. It involves a lot of logic and handles the overall execution of the program, including parsing command-line arguments, setting the working directory, constructing the configuration, building the regex pattern, and performing the search.

>>
```

In the above dialog, the model shows which files were read, and finally the model provides an answer.

This might send the current directory contents to OpenAI servers at the model's discretion,
but the model is not allowed to step outside the directory the program was run at.

## Naming

It's named so that the terminal invocation reads as natural language:
"well, what is the most complex function?.."

```
$ well what is the most complex function\?
```
