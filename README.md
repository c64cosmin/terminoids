# Asteroids game inside the terminal

![Screenshot](img/asterscr0.png)

![Screenshot](img/asterscr1.png)

### Building

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

Due to this limitation the game has two input modes:
- Sticky : This works by pressing one button, it becomes enabled until you press again\
           For example if you press forward once you keep going forward until\
           you press forward again\
           This doesn't apply to directions, only firing and forward movement\
- Non-sticky : This mean that buttons have to be tapped in order to get a move\
               holding a button down will work until another button is pressed\
               Remember this works inside the terminal, so like a text editor\
               You cannot press two buttons at once, you got to tap them.

You start with Sticky mode enabled!

Left - Right  : Rotate ship\
Up            : Move forward\
Down          : Stop movement (For sticky mode)\
Tab           : Switch between sticky & non-sticky\
Space         : Fire


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
