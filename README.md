# gestured
A simple gesture daemon in Rust

`cargo install gestured`

This is a simple gesture daemon made to watch for gesture swipes and execute a command.  It was originally made for Sway WM as a way for me to execute commands from my trackpad.

```console
# when 3 fingers are swiped from down to up run the cmd "swaymsg exec show_my_menu"
gestured -g "3,D,U,swaymsg exec show_my_menu" \
  -g "4,L,R,swaymsg workspace next" \
  -g "4,L,R,swaymsg workspace prev" \
```

# Credit

This project's CLI interface was inspired off of [lisgd](https://git.sr.ht/~mil/lisgd)
