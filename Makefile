pre-build:
	# echo "pre-build"
build: pre-build
	set -ex
	wasm-pack build --target web --out-dir ./pkg