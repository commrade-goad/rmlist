# RMLIST
a simple command line program that can read ``.rmlist`` file to play playlist of media using mpv

## BUILD
``` sh
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

## CONFIGURATION
- ``~/.config/rmlist.toml``
```toml
media_list_path = ["/home/user/Videos/"]
```
- ``/home/user/Videos/my-cool-playlist.rmlist``
```toml
media = ["/home/user/Videos/some-music-video.mp4", "/home/user/Music/some-music.mp3"]
other_flag = ["--no-video"]
```
