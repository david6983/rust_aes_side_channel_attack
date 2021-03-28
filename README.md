# Side-channel attack implementation in rust

## Hypothesis

- We start from the end
- xMin and xMax was determined on the curve with `Traces.mat`, I took the 5 last rounds

## Prerequisites

- `CTO.mat` file
- `Traces.mat` file
- [cargo](https://www.rust-lang.org/learn/get-started) to build the rust code

To quickly install rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage

1. copy `CTO.mat` and `Traces.mat` in a new folder called `res/` in the working directory
2. build the code

```
cargo build --release
```

3. launch the program

```
cargo run --release
```

or

```
./target/release/aes_side_channel
```

## Results

```
$ time cargo run --release 
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/aes_side_channel`
Initialisation...
Decrypting the key...
The decrypted key is [60, 251, 0, 146, 64, 71, 120, 185, 77, 214, 173, 176, 37, 56, 128, 1]
cargo run --release  2.81s user 0.41s system 99% cpu 3.223 total
```

## Benchmark

On a macbook pro Intel core i92,3 GHz 8 cores (Big Sur 11.2.3) - 16 giga of rams, 
the program found the 16 bytes in less than 5 seconds. In comparison, the same program with GNU octave took more than a hour.

Note: I am aware of the limitation of the GNU octave software.



