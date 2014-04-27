all: main main.dep

main.dep: main.rs Makefile
	rustc --dep-info $@ -L ~/lib -L . -g $<

main: main.rs Makefile
	rustc --dep-info $@ -L ~/lib -L . -g $<

include main.dep

.PHONY: clean

clean:
	rm -f main
