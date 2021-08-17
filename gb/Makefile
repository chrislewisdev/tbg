# todo: try to make this more cross-platform friendly

build/room-demo.gb: build/ main.asm man.2bpp
	rgbasm -o build/main.o main.asm
	rgblink -n build/room-demo.sym -o build/room-demo.gb build/main.o
	rgbfix -p0 -v build/room-demo.gb

build/:
	mkdir build

clean:
	rmdir /s /q build