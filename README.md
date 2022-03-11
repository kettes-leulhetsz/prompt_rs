This is a Rust rewrite of [fish_prompt](https://github.com/kettes-leulhetsz/fish_prompt).

### Usage

- compile & install:

```
cargo build --release --target=x86_64-unknown-linux-musl

cp target/x86_64-unknown-linux-musl/release/prompt_rs $SOMEWHERE/bin/
```

- use as your prompt:

```fish
# ~/.config/fish/functions/fish_prompt.fish

function fish_prompt --description "Write out the prompt"
	# fish doesn't export these
	prompt_rs $status $CMD_DURATION
end
```
