P=dancing_web
OBJECTS=src/main.c
EMCC=emcc
EMCC_CFLAGS=-s MAIN_MODULE=1

$(P): $(OBJECTS)
    $(EMCC) $(EMCC_CFLAGS) src/$(P).c
