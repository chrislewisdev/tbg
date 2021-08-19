# todo: try to make this more cross-platform friendly

build/room-demo.gb: build/ main.asm gen/sprite.2bpp gen/sprite.pal
	rgbasm -o build/main.o main.asm
	rgblink -n build/room-demo.sym -o build/room-demo.gb build/main.o
	rgbfix -p0 -v build/room-demo.gb

gen/sprite.2bpp: gen/ sprite.png
	rgbgfx -o gen/sprite.2bpp sprite.png

gen/sprite.pal: gen/ sprite-palette.png
	rgbgfx -p gen/sprite.pal sprite-palette.png

build/:
	mkdir build

gen/:
	mkdir gen

clean:
	rmdir /s /q build
	rmdir /s /q gen