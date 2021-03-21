
SERVER:=snake_server/target/debug/snake_server
CLIENT:=snake_client/target/debug/snake_client

all: $(SERVER) $(CLIENT) 

$(SERVER): snake_server/src/*.rs
	cd snake_server && cargo build

$(CLIENT): snake_client/src/*.rs
	cd snake_client && cargo build

# Demo for 1 player game
demo1: all
	cd snake_server && cargo run 4 3 &> /dev/null &
	cd snake_client && cargo run 1

demo2: all
	cd snake_server && cargo run 4 2 &> /dev/null &
	cd snake_client && cargo run 2


