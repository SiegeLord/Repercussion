all: main main.dep

main.dep: main.rs Makefile
	rustc -O --dep-info $@ -L ~/lib -g $<

main: main.rs Makefile
	rustc -O --dep-info $@ -L ~/lib -g $<

include main.dep

.PHONY: clean

clean:
	rm -f main
