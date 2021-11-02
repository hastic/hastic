all: server client
	mkdir release
	cp server/target/x86_64-unknown-linux-musl/release/hastic release
	mkdir release/public
	cp -r client/dist/* release/public/
	cp server/config.example.toml release/config.toml 


server:
	cd server;cargo build --release --target x86_64-unknown-linux-musl

client client/dist: 
	cd client;yarn build

clean:
	rm -r release
	rm -r client/dist
	rm -r server/target/release

.PHONY: server client all
