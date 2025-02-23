run:
	sudo nice -n -20 cargo run --jobs 1


bench:
	cargo bench --jobs 1