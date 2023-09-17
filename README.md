# nu-plugin-bexpand

Bash-style brace expansion in nushell.

## Format and Functionality

The format specification and functionality is fully explained in [the bexpand
crate](https://github.com/Taywee/bexpand), which implements the parser and
iterator types independent of nushell.

## Usage

```nushell
> cargo install nu-plugin-bexpand
...

> register ~/.cargo/bin/nu_plugin_bexpand

> help str bexpand
Bash-style brace expansion

Usage:
  > str bexpand 

Flags:
  -h, --help - Display the help message for this command

Signatures:
  <string> | str bexpand -> list<string>
  list<string> | str bexpand -> list<string>

Examples:
  Get a list of standard nushell config items
  > '~/config/nushell/{env,config,plugin}.nu' | str bexpand

> 'a{b,c,d{🥰..🥴..2}e}f' | str bexpand
╭───┬────────╮
│ 0 │ abf    │
│ 1 │ acf    │
│ 2 │ ad🥰ef │
│ 3 │ ad🥲ef │
│ 4 │ ad🥴ef │
╰───┴────────╯
 
> 'a{1..3}c{4..6..2}b' | str bexpand
╭───┬───────╮
│ 0 │ a1c4b │
│ 1 │ a1c6b │
│ 2 │ a2c4b │
│ 3 │ a2c6b │
│ 4 │ a3c4b │
│ 5 │ a3c6b │
╰───┴───────╯

> [a{🤐..🤒}b c{🧛..🦜..31}d e{15..-23..8}f] | str bexpand
╭────┬───────╮
│  0 │ a🤐b  │
│  1 │ a🤑b  │
│  2 │ a🤒b  │
│  3 │ c🧛d  │
│  4 │ c🦼d  │
│  5 │ c🦝d  │
│  6 │ e15f  │
│  7 │ e7f   │
│  8 │ e-1f  │
│  9 │ e-9f  │
│ 10 │ e-17f │
╰────┴───────╯

> '{0..🧿..129300}' | str bexpand
╭───┬────╮
│ 0 │ 0  │
│ 1 │ 🥄 │
╰───┴────╯

> ['success{a,b,c}test' 'fail{test'] | str bexpand
Error:   × Brace expression failed to parse
   ╭─[entry #13:1:1]
 1 │ ['success{a,b,c}test' 'fail{test'] | str bexpand
   ·                       ─────┬─────
   ·                            ╰── 0: at line 1, in Eof:
fail{test
    ^


   ╰────

> "{\u{D7FF}..\u{E000}}" | str bexpand
Error:   × Expression failed to generate
   ╭─[entry #15:1:1]
 1 │ "{\u{D7FF}..\u{E000}}" | str bexpand
   ·                          ─────┬─────
   ·                               ╰── converted integer out of range for `char`
   ╰────
```
