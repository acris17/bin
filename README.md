# bin
Manage the installation of personal programs.

## Examples
```$ bin install hello```

```$ bin uninstall hello```

```$ bin list```


## Build
1. Run make in the terminal.

```$ make```

2. Ensure the program is in release directory.

```./target/release/bin```


## Configuration
Environment variables:

```BIN_DIR # Where to install your programs```

Example configuration:

```export BIN_DIR="~/Development/bin"```
