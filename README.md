# hackassembler

A Rust implementation of an assembler for the [Hack assembly language](https://www.nand2tetris.org/project06) defined in awesome NAND2Tetris textbook.

## Install

```
$ cargo build --release
$ cp ./target/release/hackassembler /usr/local/bin
```

## Usage

```
$ hackassembler ./test/pong/Pong.asm
```

## Test

```
$ cargo test
```