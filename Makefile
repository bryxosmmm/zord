build:
	cd app/ && cargo build --release

install:
	mkdir -p ${HOME}/.config/zord/
	cp ./default.toml ${HOME}/.config/zord/config.toml
	cd app/ && \
	chmod +x ./target/release/app && \
	sudo cp ./target/release/app /usr/bin/zord && \
	sudo cp -r ./public /usr/share/zord

