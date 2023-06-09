# RMLIST
a simple command line program that can read ``.rmlist`` file to play playlist of media using mpv

## BUILD
```sh
$ cargo build --release
```

## USAGE
- create playlist
```sh
$ ./rmlist create /path/to/playlist
```

- play playlist using absolute path or relative
```sh
$ ./rmlist play /path/to/playlist.rmlist
```

- play playlist using filename if dir is at configuration file
```sh
$ ./rmlist play my-cool-playlist
```

## NOTE
for now to populate ``media`` and ``other-flag`` is needed to edit the ``.rmlist`` file manually.
