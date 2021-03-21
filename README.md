# Introduction
A small multiplayer snake game written in Rust for learning purpose.
- `snake_server` is the server-side of the game
- `snake_client` is the client-side, multiple instances can be executed
	to connect to a game hosted by an instance of `snake_server`

# Requirements
- Rust environment and cargo
- This project uses the Rust crate _termion_ which supports
 Redox, Mac OS X, and Linux (or, in general, ANSI terminals).
	
# Compile the projects
Use the makefile:
> make

# Start demos
- 1 player game with 4 snakes
	> make demo1
- 2 players game with 4 snakes
	> make demo2

# Start a game manually
1. Launch binary from `snake_server`
	> cargo run			// in snake_server directory 
	- Without any arguments you will have a game with 4 snakes
		which 3 of them are bots
	- You can add command line arguments
		* for a game with 3 snakes and 1 bot
		> cargo run 3 1
	- The game won't start until enough "human" players join
2. Launch binary from `snake_client`
	> cargo				// in snake_client directory
	- Without any arguments your terminal will host one player
	- You can also play with 2 players on the same terminal
		> cargo run 2

