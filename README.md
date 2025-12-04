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

## adding to path
1. type "edit the system environment variables" in search
2. click on "advanced" tab and click on "Environment Variables..."
3. Under "System variables" click on "Path" and "Edit..."
4. Create a new variable and add the executable path to the field
