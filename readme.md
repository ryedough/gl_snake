# GL Snake
implementation of the classic Snake game in Rust With Opengl.

## Motivation
After i've done reading [learnopengl](https://learnopengl.com/) and [Game Programming Algorithms and Techniques: A Platform-Agnostic Approach](https://www.goodreads.com/book/show/18058161-game-programming-algorithms-and-techniques), i want to create a simple game from scratch using as few library as possible, i also build it in parts to learn how to do 2d calculation based opengl rendering (the learnopengl book doesn't cover this in depth), and writing a more meaningful program in rust, rather than some simple algorithm.

## Implemented using library :
- [glow](https://github.com/grovesNL/glow) (opengl platform agnostic binding for rust)
- [glutin](https://github.com/rust-windowing/glutin) (opengl context creation)
- [winit](https://github.com/rust-windowing/winit) (window creation and management)