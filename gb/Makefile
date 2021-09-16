
### CORE ###

build/tbg.gb: build/ main.asm gen/sprite.2bpp gen/sprite.pal gen/tiles.2bpp gen/tiles.pal
	rgbasm -o build/main.o main.asm
	rgblink -n build/tbg.sym -o build/tbg.gb build/main.o
	rgbfix -p0 -v build/tbg.gb

### ASSETS ###

gen/sprite.2bpp: gen/ assets/sprite.png
	rgbgfx -o gen/sprite.2bpp assets/sprite.png

gen/sprite.pal: gen/ assets/sprite-palette.png
	rgbgfx -p gen/sprite.pal assets/sprite-palette.png

gen/tiles.2bpp: gen/map.png
	rgbgfx -u -t gen/map.tilemap -o gen/tiles.2bpp gen/map.png

gen/map.png: gen/ assets/map.tmx
	tmxrasterizer assets/map.tmx gen/map.png

gen/tiles.pal: gen/ assets/tiles-palette.png
	rgbgfx -p gen/tiles.pal assets/tiles-palette.png

### MISC ###

build/:
	mkdir build

gen/:
	mkdir gen

run-gb: build/tbg.gb
	bgb64 -set SystemMode=0 -watch build/tbg.gb

run-gbc: build/tbg.gb
	bgb64 -set SystemMode=1 -watch build/tbg.gb

clean:
	rm -rf build
	rm -rf gen