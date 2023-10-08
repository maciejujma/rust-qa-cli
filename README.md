## Rust Question Answering CLI
This is a simple CLI answering questions based on the context using Bert model.
Context can be either provided by the user or loaded from the Wikipedia webpage.

## Examples

### Answering based on the provided context
To get answer based on the Wikipedia webpage use `--context` command.
```bash
cargo run -- --question "Where does Amy live?" --context "Amy is 32 years old actress from New York"
```
Result:
```bash
Question: Where does Amy live?
Answer: New York
```

### Answering based on the Wikipedia webpage
To get answer based on the Wikipedia webpage use `--wikipedia` command.
```bash
cargo run -- --question "What instrument did Chopin play?" --wikipedia "Frédéric Chopin"
```
Result:
```bash
Question: What instrument did Chopin play?
Answer: piano
```

## Help
To get the application description, usage syntax and the list of options use `cargo run -- --help`