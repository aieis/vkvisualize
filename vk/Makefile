CC=gcc
CFLAGS=-std=c17
IDIRS=-I/usr/include/GLFW/
LDFLAGS=-L/usr/lib/  -lglfw -lrt -lm -lX11 -lpthread -lxcb -lXau -lXdmcp -lvulkan

main: main.c
	$(CC) $(CFLAGS) main.c -o main $(IDIRS) $(LDFLAGS)

.PHONY: test clean

test: main
	./main

clean:
	rm -f main
