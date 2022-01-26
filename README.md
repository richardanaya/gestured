# gestured
A simple gesture daemon in Rust

This is a simple gesture daemon made to watch for gesture swipes and execute a command.  It was originally made for Sway WM as a way for me to execute commands from my trackpad.

```
gestured -g "3,D,U,swaymsg exec show_my_menu"
```

# Credit

This project's CLI interface was inspired off of [lisgd](https://git.sr.ht/~mil/lisgd)
