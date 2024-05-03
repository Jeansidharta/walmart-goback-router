build:
	wasm-pack build --target web
	mkdir ../walmart-goback-frontend/tsp-pkg/
	mv pkg/* ../walmart-goback-frontend/tsp-pkg/
