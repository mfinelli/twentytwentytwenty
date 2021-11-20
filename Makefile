all: target/release/twentytwentytwenty LICENSE-THIRD-PARTY

LICENSE-THIRD-PARTY: Cargo.lock about.hbs about.toml
	cargo about generate about.hbs | python3 unescape.py > $@

target/release/twentytwentytwenty: src/main.rs Cargo.lock
	cargo build --release --locked

pkg/twentytwentytwenty.png: pkg/twentytwentytwenty.xcf
	magick $< -flatten $@

clean:
	rm -rf LICENSE-THIRD-PARTY target

.PHONY: all clean
