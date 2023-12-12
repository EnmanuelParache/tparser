# TPARSER
Simple cli to parse time string `hh:mm:ss` to hours, minutes or seconds.

## Build
```shell
cargo build
```

## Test
```shell
cargo test
```

## Run
After building the project

### To hours (default)
```shell
./target/debug/tparser -t 23:59:59 
Time { hour: 23, minutes: 59, seconds: 59 }
23.999722
```

### To minutes 
```shell
Time { hour: 23, minutes: 59, seconds: 59 }
1439.9833
```

### To seconds
```shell
./target/debug/tparser -t 23:59:59 -u 2
Time { hour: 23, minutes: 59, seconds: 59 }
86399
```