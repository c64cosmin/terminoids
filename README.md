# Asteroids game inside the terminal

![Title](img/title.png)

![Screenshot](img/asterscr0.png)

![Screenshot](img/asterscr1.png)

![Screenshot](img/asterscr2.png)

### Building

[You will need Rust to compile the binary](https://www.rust-lang.org/tools/install)

```
cargo build --release
```

### Running

```
./target/release/terminoids
```

### Gameplay

Unfortunately terminals don't directly support input like UI apps do.\
What I mean is that KEY_UP & KEY_DOWN events are not supported,\
the way the terminal works is to get a key stroke event and act accordingly.

This means that buttons have to be tapped in order to get a move\
holding a button down will work until another button is pressed\
Remember this works inside the terminal, so like a text editor\
You cannot press two buttons at once, you got to tap them.

Due to this limitation the game work like this:
- Pressing **Fire** button will enable firing, the ship will fire automatically\
  until the **Fire** is pressed again to stop
- Pressing **Left** or **Right** will turn the ship only a slight amount
- Pressing **Up** will propel the ship forward a slight amount


Left - Right  : Rotate ship\
Up            : Move forward\
Space         : Fire\
P             : Pause\
Q or Ctrl-C   : Exit


#### Asteroids

- Huge           : 200pts

![Preview](img/aster_a_0.png)

- Big            : 150pts

![Preview](img/aster_a_1.png)

- Medium         : 100pts

![Preview](img/aster_a_2.png)

- Small          :  75pts

![Preview](img/aster_a_3.png)

- Tiny           :  50pts

![Preview](img/aster_a_4.png)


#### Star-ships

- Big cluster    : 250pts

![Preview](img/aster_s_0.png)

- Medium cluster : 200pts

![Preview](img/aster_s_1.png)

- Small cluster  : 150pts

![Preview](img/aster_s_2.png)

- Flying ship    : 100pts

![Preview](img/aster_s_3.png)


#### Power-ups

- Red            : 1000pts - gives piercing bullets for 5 seconds

![Preview](img/aster_p_0.png)

- Green          : 1000pts - gives split fire for 7 seconds

![Preview](img/aster_p_1.png)

- Blue           : 1000pts - gives shield for 10 seconds

![Preview](img/aster_p_2.png)

- White          : 1000pts - gives rapid fire for 5 seconds

![Preview](img/aster_p_3.png)


## Self-Promotion

I make video games

[Twitter](http://twitter.com/c64cosmin)

[YouTube](https://www.youtube.com/@c64cosmin)

You can play some free games here\
No download, directly in browser\
Hope you get a high score in the leader board :D

[HomePage](https://stupidrat.com)

## License

Copyright (c) Cosmin MUNTEANU.
