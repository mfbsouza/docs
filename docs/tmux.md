# Using tmux

tmux is a terminal multiplexer. It lets you easily switch between several programs
in one terminal, detach them (keep them running in the background) and reattach them
to a different terminal. It's a must have tool for SysAdmin and others working from
a ssh session.

## Launch tmux

Running tmux is simply as running:

	$ tmux

## Creating new windows and moving through then

Hit `[Ctrl]+[b] and then [c]` to create a new windows
Hit `[Ctrl]+[b] and then [n] or [p]` to move the the next or previous window
Hit `[Ctrl]+[b] and some number` to move to a specific window by number

## Detach/Reattach from the current session

Hit `[Ctrl]+[b] and then [d]` to detach from the session

you can now safely log out and all processes will keep running in the background.

To reattach simply run:

	$ tmux attach

or:

	$ tmux a

this will attach to your detached tmux session.

**Tip:** You can run `tmux ls` to list all running sessions

## Spliting window in panes and moving through them

Hit `[Ctrl]+[b] and then [%]` to split the window horizontally
Hit `[Ctrl]+[b] and then ["]` to split the window vertically
Hit `[Ctrl]+[b] and any Arrow Key` to move between panes
Hit `[Ctrl]+[b] and then [x]` to close pane

## List all keybindings

Hit `[Ctrl]+[b] and then [?]` to view all keybindings. Press [q] to exit.
