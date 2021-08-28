P=dancing_web
OBJECTS=src/main.c
EMCC=emcc
EMCC_CFLAGS=-s MAIN_MODULE=1
BUILDDIR=build

$(P): $(OBJECTS)
	$(EMCC) $(EMCC_CFLAGS) src/main.c -o $(BUILDDIR)/a.out.js