run:
	sudo nice -n -20 cargo run --jobs 1

bench:
	cargo bench --jobs 1

setup:
	curl https://sh.rustup.rs -sSf | sh
	. "$$HOME/.cargo/env"

push:
	git add -A
	git commit -m "update"
	git push space