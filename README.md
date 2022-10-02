# gl-objects-rs


## Run with glfw

cargo run

## Run with sdl2

cargo run --features sdl2



Setting up SDL2

### macOS
#### Homebrew
On macOS, it's a good idea to install these via
[homebrew][homebrew].

```
brew install sdl2
```

In recent versions of Homebrew, the installed libraries are usually linked into `$(brew --prefix)/lib`.
If you are running an older version, the symlink for SDL might reside in `/usr/local/lib`.

To make linking libraries installed by Homebrew easier, do the following for your respective shell.

Add this line to your `~/.zshenv` or `~/.bash_profile` depending on whether you use ZSH or Bash.
```
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

### Linux and Windos

<a href="https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md">Check out docs</a>
