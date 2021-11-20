LICENSE-THIRD-PARTY: Cargo.lock about.hbs about.toml
	cargo about generate about.hbs | python3 unescape.py > $@
