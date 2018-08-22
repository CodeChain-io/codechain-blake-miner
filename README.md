# CodeChain Blake Miner [![Build Status](https://travis-ci.org/CodeChain-io/codechain-blake-miner.svg?branch=master)](https://travis-ci.org/CodeChain-io/codechain-blake-miner)

Mining worker for blake PoW algorithms in [CodeChain](https://github.com/CodeChain-io/codechain).

## Build

CodeChain blake miner is written in Rust. We recommend setting up a build environment with [rustup](https://rustup.rs/).

To build an executable in release mode, run the following command.
```
cargo build --release
```

The resulting binary file can be found at `target/release/codechain-blake-miner`.

## Usage
```
codechain-blake-miner [OPTIONS]
```

### Usage Examples
* listening to port **3333**, submitting to port **8080** :
```
codechain-blake-miner -p 3333 -s 8080
```

## Configuration

| Option | Description                    | Default | Required |
| :----: | ------------------------------ |:-------------:|:--------:|
| `-p`   | Port number to receive job     | 3333 | No |
| `-s`   | Port number to submit solution | 8080 | No |
