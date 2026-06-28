PREFIX ?= /usr/local
BINARY  = xof-fighter

.PHONY: build install uninstall update

build:
	cargo build --release

install: build
	install -Dm755 target/release/$(BINARY) $(DESTDIR)$(PREFIX)/bin/$(BINARY)

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/$(BINARY)

update:
	git pull
	$(MAKE) install
