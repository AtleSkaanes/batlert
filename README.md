## BATLERT

A low battery popup for linux, made with GTK.
I made this since I wanted something other than a notification to indicate critical battery status.
I am not sure on how many devices, DE's, or WM's it will run, I have only tested it on Hyprland, but if you run into any errors, you are more than welcome to make an issue or submit a PR.

### Installing

You can install it by cloning the repo locally, and compile it

```sh
$ git clone https://github.com/AtleSkaanes/batlert
$ cd batlert
$ cargo build --release # or cargo install --path .
```

## Usage

Just run the program, and it will automatically popup when the battery level is critical.
By default it pops up at 5% battery, but you can specify what battery % it should pop up at yourself, with the `--battery-pct` argument.

```sh
$ batlert --battery-pct [INT]
```

You can look at all the programs arguments by passing the `--help` flag.

To make the program launch automatically at startup, you can put it in your WM's or DE's startup routine.
Like in hyprland you can do:

```hypr
exec-once = batlert
```

Now whenever the battery level gets critical, you will see this popup:

![Popup example](images/popup.png)
