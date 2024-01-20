build:
	wasm-pack build --target web && mv pkg/* ../front-end/tsp-pkg/
