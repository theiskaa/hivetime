# Hivetime

Hivetime is a straightforward interpreter and calculator designed for short-time syntax, primarily used in Jira work logs.

### Installation

To install Hivetime REPL, you need to build the REPL yourself. Follow these steps:

1. Navigate to the `repl` directory.
2. Run the `cargo build` command.
3. Once the build is complete, you can start using Hivetime.

### Usage

You can perform calculations using the following syntax:
```
42m + 18m
```

The result will be displayed as 1h or (60m).

Alternatively, you can also use the following syntax:
```
42 minutes + 18 minutes
```

---

### Error Handling

Hivetime provides error handling in the lexer, parser, and calculator components, ensuring that any errors occurring during the interpretation and calculation process are appropriately handled and reported. However, please note that error handling is currently not implemented in the REPL itself.
