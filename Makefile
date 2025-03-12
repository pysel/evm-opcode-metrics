run-pi:
	CARGO_TARGET_DIR=/mnt/sda/target nice -n -20 cargo run --jobs 1

run:
	nice -n -20 cargo run --jobs 1
bench:
	cargo bench --jobs 1 > benchmark_output.txt

setup:
	curl https://sh.rustup.rs -sSf | sh
	. "${HOME}/.cargo/env"
	
setup-pi: setup
	mkdir /mnt/sda
	mount /dev/mmcblk0p2 /mnt/sda
	mv ~/.cargo/registry /mnt/sda/cargo/
	export CARGO_HOME="${HOME}/.cargo"
	export CARGO_REGISTRY_DIR="/mnt/sda/cargo/registry"
	export CARGO_TARGET_DIR="/mnt/sda/target"
	source ~/.bashrc

push:
	git add -A
	git commit -m "(minor auto) update"
	git push space

parse:
	python3 parser.py

fetch-benchmark:
	scp springfield:benchmark_output.txt benchmark_output.txt

fetch:
	scp springfield:avg-opcode-cycles-arm.csv avg-opcode-cycles-arm.csv
