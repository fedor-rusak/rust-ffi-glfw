# Rust + FFI + GLFW

Want some fun with Rust and game dev? Well bad news everyone. Everything for rust is unix-oriented and the top gaming OS got short stick.

And I want to do simple example with GLFW using Rust bit by bit. So that it would work and could be compiled on modern Windows PC for f%&$ sake!

# Some thoughts

Using native stuff is little hell in almost any other language be it java or NodeJS runtime. Remember that when you use DLLs you need corresponding lib-files.

And please remember that your DLLs and LIBs should be compiled with similar compiler as you use for Rust right now. My included natives are done with Microsoft Visual Studio Compiler. Remember that some time ago 32-bit processors were thing. I provided only for 64-bit ones.

And for those who think why code looks like entangled hieroglyphic madness. Idea is somewhat simple. OS provides as API for handling input and Window-things like moving  or actually showing them.

I don't want to call OS API directly so I use GLFW, it is technically a wrapper. But when I want to make some things like set background color I have to use GLEW! Because this is a special thing for working with OpenGL!

That is right. Want a window with green background and see some internals? Please be ready to see some tech guts of all the interactions between libraries, wrappers and etc.

It would be nice to get some proper module usage and provide some type conversion so that c_int, etc. are not used everywhere. But I don't know when this would happen. Enjoy!

Now at least I can hide all monstrosity inside module. And now I know that if you depend on some git repo you should force cargo to download its source. Dammit!

# Windows instructions from scratch

 * Install Rust, Cargo with [rustup](https://www.rust-lang.org/en-US/install.html)
 * Install [Visual C++ Builds Tools 2015 or later](https://visualstudio.microsoft.com/ru/thank-you-downloading-visual-studio/?sku=BuildTools&rel=15)

Run these commands:

```
cargo update
cargo build
cargo run
```

PS Behind proxy set environment variables *http_proxy* and *https_proxy*.

# Libraries used

 * [GLFW](https://github.com/glfw/glfw) - zlib/libpng License
 * [GLEW](https://github.com/nigels-com/glew) -  Modified BSD License, Mesa 3-D License (MIT License), and the Khronos License (MIT License)
 * https://github.com/fedor-rusak/first_lib - my first library with no License for a moment. So I guess it is not open source...

# History

## 0.0.2
  - all fancy stuff inside external module. Yet there are native dependencies inside this repo.

## 0.0.1
  - initial version with all code in main. Plus some dependency on external custom crate.