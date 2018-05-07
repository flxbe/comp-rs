# comp-rs

A compositing window manager for linux written in rust. This is a personal, educational project.

### Requirements

* Add current user to groups `video` and `input`.

### Roadmap

###### Display access

- [x] Access the display by creating a `DRM` framebuffer and registering it.
- [ ] Hold on to display access.

###### Mouse input

- [x] Read and parse the mouse input events by reading the appropriate device file.
- [ ] Update the internal state of the mouse.

###### Multithreading

- [ ] Use some runtime such as `tokio` to concurrently handle all events.

###### Keyboard input

- [ ] Read and parse the keyboard input events by reading the appropriate device file.
- [ ] Update the internal state of the keyboard.

###### Window server

- [ ] Create a socket for other applications to connect to.
- [ ] Window registration.
- [ ] Resizing.
- [ ] Viewport updates.
- [ ] Forward input events to the applications.
