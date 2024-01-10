# TPARSER
Simple CLI to parse time string `hh:mm:ss` to hours, minutes or seconds.

## Run tests
```shell
cargo test
```

## Build
```shell
cargo build
```

## Build for release
```shell
cargo build --release
```

## Install in your path

### Mac OS (zsh)

Just add the following line to `.zprofile` replace `pathToRepo` with actual path to where you clone the project. Or simply copy the `tparser` binary where you want it to be installed and add that path instead.

```bash
# Add tparser
export PATH="$PATH:/pathToRepo/tparser/target/release/"

```

## Run
After building and installing the project 

### To hours (default)
```shell
tparser 23:59:59 
23.999722
```

### To minutes 
```shell
tparser 23:59:59  -u m
1439.9833
```

### To seconds
```shell
tparser 23:59:59 -u s
86399
```

### Decompose
Using the `--decompose`  `-d` option will expect a floating point number or integer as time argument and print the value in `hh:mm:ss` format based on the unit passed by default is hours (h)

#### Decomposing hours
```shell
tparser 60.5085 -d
60:30:30
```

#### Decomposing minutes
```shell
tparser 60 -d -u m
1:0:0
```

#### Decomposing seconds
```shell
tparser 3661 -d -u s
1:1:1
```

### Debug
When running debug build additional information (Time struct) will be printed

```shell
./target/debug/tparser 23:59:59 -u s            
Time { hour: 23, minutes: 59, seconds: 59 }
86399
```

or with cargo run

```shell
cargo run -- 23:59:59 -u s
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/tparser '23:59:59' -u s`
Time { hour: 23, minutes: 59, seconds: 59 }
86399
```

