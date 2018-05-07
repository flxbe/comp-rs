# comp-rs

A compositing window manager for linux written in rust. This is a personal, educational project.

## Requirements

* Add current user to groups `video` and `input`.

## Roadmap

#### Display access

Access to the display by creating a `DRM` framebuffer and registering it.

#### Mouse input

Read the mouse input events by reading the appropriate device file.

#### Multithreading

Use some runtime such as `tokio` to concurrently handle all events.

#### Keyboard input

The same as for the mouse input.

#### Window server

Create a socket for other applications to connect to. It should be possible to register new windows, to change the window size and to update the viewport. All input signals should be proxied to the relevant windows.


