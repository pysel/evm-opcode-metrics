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

push:
	git add -A
	git commit -m "(minor auto) update"
	git push space

parse:
	python3 parser.py