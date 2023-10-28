# `eve` - Your AI Terminal Assistant

`eve-cli` is a command line tool written in Rust. She is your development assistant for debugging errors and working out commands.

![eve-demo](https://github.com/ted-jenks/eve-cli/assets/77486246/6878cdfd-5997-46f6-bfb1-ffda3693fb09)

To use `eve` you will need a valid OpenAI API key. This can be generated [here](https://platform.openai.com/account/api-keys).

## Usage

### `hello`

```
eve hello
```

Say hello to `eve`. Get a newly generated response each time.

### `config`

```
eve config
```

Set `eve` up. She will ask you to provide your API key and the model you wish to use for OpenAI queries.

### `command`

```
eve command "Create a directory called test and create a file called wow.txt and then open it with vim"
```

```
eve command --yolo "Force remove the root directory"
```

Get command suggestions from `eve` and optionally run them. Use the `--yolo` flag to run the commands without use confirmation.

### `error`

```
cargo build |& eve error
```

Pipe errors from compilers, programs, and more into eve to get an explanation and suggested fix (note, you probably want to use `|&` so you get stdout and stderr)

### `--help`

```
Eve is your personal command-line AI assistant.

Usage: eve [COMMAND]

Commands:
  hello    Say Hello!
  config   Configure eve with you OpenAI API key and choice of model
  command  Get a command to complete a task defined in natural language
  error    Get an explanation and suggested fix for an error message
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Building `eve`

Building `eve` is easy. Just run:

```
cargo build --release
```

To get an `eve` executable in your target directory.
