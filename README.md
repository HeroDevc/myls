# myls
Basic and shitty remake of ls command in rust.

## building
```
cargo build --release
```

## usage
```
myls path_to_files -l
```

-l argument is used for better formatted output with file sizes
-r is optional with -l, doesn't work without -l
