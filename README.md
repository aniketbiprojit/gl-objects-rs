# gl-objects-rs

[![crates](https://img.shields.io/crates/v/gl_object.svg?style=for-the-badge&label=gl_object&color=brightgreen)](https://crates.io/crates/gl_object)
![LICENSE](https://img.shields.io/badge/license-AGPL%202.0-blue.svg?style=for-the-badge)


Usage as Library:

## For Primitives

```rs

let rectangle = &mut Rectangle::new(200 //width, 
                                    200, // height
                                    "<path-to-glsl-shader>.shader");
'render: loop { 
    rectangle.attach(&gl);

    if window.resized == true {
        rectangle.window_resize(draw_size,size);
    }

    if key == "space" {
        rectangle.move_model(0,2.0,0.0);
    }
    rectangle.render(&gl);
}
rectangle.detach(&gl);
```

## For Shaders

Note: Automatically adds GLSL version.

```rs
let shaders = ShaderData::new(source);

let shader_sources = [
    (VERTEX_SHADER_INT, shaders.vertex_shader.source),
    (VERTEX_SHADER_INT, shaders.fragment_shader.source),
];
```

## Run with glfw

```sh
cargo run
cargo run --example glfw
```

## Run with sdl2

```sh
cargo run --features sdl2
cargo run --example sdl2 --features sdl2
```


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

### Linux and Windows

<a href="https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md">Check out docs</a>


### Sponsor

[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/aniketbiprojit)