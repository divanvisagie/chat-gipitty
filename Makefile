install:
	cargo build --release
	cp target/release/cgip /usr/local/bin/
	cp docs/cgip.1 /usr/local/share/man/man1/

man:
	groff -man -Tascii docs/cgip.1 | less
