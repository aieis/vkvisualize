THIRD_PARTY = C:/lib/vcpkg/installed/x64-windows

INC = -I$(THIRD_PARTY)/include -Iinclude -Ithird_party/include
LIB = -L$(THIRD_PARTY)/lib -lglfw3dll -lopengl32 -luser32 -lgdi32 -lkernel32 -lshell32 -lrealsense2
#LIB = -lglfw -lGL -lm
CC = gcc
LD = gcc

CCFLAGS = $(INC) -DCGLM_STATIC=ON -DCGLM_SHARED=OFF
LDFLAGS = $(LIB)

build/pcl-viewer: build/main.o build/shader.o build/compass.o build/glad.o build/pcl.o build/record.o build/record_player.o build/input.o
	$(LD) -o $@ $^ $(LDFLAGS)

build/main.o: src/main.c src/state.h src/graphics/shader.h src/graphics/pcl.h src/utils/record.h src/device/record_player.h src/input.h
	$(CC) -c -o $@ $< $(CCFLAGS)

build/input.o: src/input.c src/input.h src/state.h
	$(CC) -c -o $@ $< $(CCFLAGS)

build/shader.o: src/graphics/shader.c src/graphics/shader.h
	$(CC) -c -o $@ $< $(CCFLAGS)

build/compass.o: src/graphics/compass.c src/graphics/compass.h
	$(CC) -c -o $@ $< $(CCFLAGS)

build/pcl.o: src/graphics/pcl.c src/graphics/pcl.h
	$(CC) -c -o $@ $< $(CCFLAGS)

build/glad.o: src/glad.c
	$(CC) -c -o $@ $< $(CCFLAGS)

build/record.o: src/utils/record.c src/utils/record.h
	$(CC) -c -o $@ $< $(CCFLAGS)

build/record_player.o: src/device/record_player.c src/utils/record.h
	$(CC) -c -o $@ $< $(CCFLAGS)

build/record.oo: tools/record.c src/utils/record.h
	$(CC) -c -o $@ $< $(CCFLAGS)

build/record: build/record.oo build/record.o
	$(LD) -o $@ $^ $(LDFLAGS)

all: build/record build/pcl-viewer

run: build/pcl-viewer
	@echo "Running: " $^
	@$^	

clean:
	rm build/*.o
	rm build/pcl-viewer
	rm build/record
