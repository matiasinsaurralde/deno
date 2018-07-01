TS_FILES = \
	console.ts \
	deno.d.ts \
	deno.ts \
	dispatch.ts \
	globals.ts \
	main.ts \
	msg.pb.d.ts \
	msg.pb.js \
	os.ts \
	runtime.ts \
	text-encoding.d.ts \
	tsconfig.json \
	types.ts \
	url.js \
	util.ts \
	v8_source_maps.ts \
	v8worker2.d.ts \
	msg.capnp.ts

RUST_FILES = \
	src/main.rs \
	src/runtime.rs


deno:	$(RUST_FILES)
	cargo build

msg.pb.js: src/msg.proto node_modules
	./node_modules/.bin/pbjs -t static-module -w commonjs -o msg.pb.js src/msg.proto

msg.pb.d.ts: msg.pb.js node_modules
	./node_modules/.bin/pbts -o msg.pb.d.ts msg.pb.js

assets: $(TS_FILES) node_modules
	./node_modules/.bin/tsc --noEmit # Only for type checking.
	./node_modules/.bin/parcel build --out-dir=dist/ --log-level=1 --no-minify main.ts
	cp node_modules/typescript/lib/*d.ts dist/
	cp deno.d.ts dist/

node_modules:
	yarn

clean:
	-rm -f target/debug/deno msg.pb.js msg.pb.d.ts
	-rm -rf dist/

distclean: clean
	-rm -rf node_modules/

lint: node_modules
	yarn lint

fmt: node_modules
	yarn fmt
	clang-format src/msg.proto -i

test: deno
	cargo test -- --nocapture

.PHONY: test lint clean distclean
