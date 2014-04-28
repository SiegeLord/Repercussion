all: repercussion main.dep

main.dep: repercussion.rs Makefile
	rustc --dep-info $@ -L ~/lib -L . -g $<

repercussion: repercussion.rs Makefile
	rustc --dep-info $@ -L ~/lib -L . -g $<

include main.dep

.PHONY: clean

clean:
	rm -f repercussion
	rm -f main.dep
