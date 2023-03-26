
## pobsd
pobsd is a TUI written in Rust designed to browse the PlayOnBSD
database that can be find [here](https://github.com/playonbsd/OpenBSD-Games-Database)

It makes use of both `plege(2)` and `unveil(2)`.

### Installing
You can install it using `cargo` with `cargo install pobsd`.
Make sure to update to your `$PATH` to be able to use 
it (usually by adding `$HOME/.cargo/bin`).

### Browsing
The last (but not the least) function is the browse function.
It provides a nice yet simple terminal interface for browsing
and searching the database.
[![asciicast](https://asciinema.org/a/563130.svg)](https://asciinema.org/a/563130)

### Credits
Reading [https://github.com/graymind75/passmng](https://github.com/graymind75/passmng) source
code helped me a lot to implement the TUI.

