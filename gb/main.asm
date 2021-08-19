INCLUDE "hardware.inc"

SECTION "ROM Title", ROM0[$0134]
  DB "Room Demo"

SECTION "Nintendo Logo", ROM0[$0104]
  NINTENDO_LOGO

SECTION "Entrypoint", ROM0[$0100]
  nop
  jp Startup

SECTION "Gameboy Colour Support", ROM0[$0143]
  DB CART_COMPATIBLE_DMG_GBC

SECTION "Graphics data", ROM0
TileData:
INCBIN "gen/tiles.2bpp"
EndTileData:
SpriteData:
INCBIN "gen/sprite.2bpp"
EndSpriteData:
MapData:
INCBIN "gen/room.tilemap"
EndMapData:
SpritesColourPaletteData:
INCBIN "gen/sprite.pal"
EndSpritesColourPaletteData:
TileColourPaletteData:
INCBIN "gen/tiles.pal"
EndTileColourPaletteData:

SECTION "Game code", ROM0[$0150]
Startup:
  call WaitForNextVerticalBlank
  call DisableLcd
  call ClearGraphicsData
  call LoadGfx
  call InitialiseRoom
  call InitialiseSprite
  call InitialiseMonochromePalettes
  call InitialiseColourPalettes
  call EnableLcd
GameLoop:
  nop
  jp GameLoop

SECTION "Functions", ROM0
WaitForNextVerticalBlank::
.untilVerticalBlank
  ld a, [rLY]
  cp 144
jr nz, .untilVerticalBlank
ret

; hl = destination address
; bc = no. bytes to zero
ZeroMemory::
.untilAllBytesAreZeroed
  ld [hl], $00
  inc hl
  dec bc
  ld a, b
  or c
jr nz, .untilAllBytesAreZeroed
ret

; hl = destination address
; de = source address
; bc = no. bytes to copy
CopyMemory::
  .untilAllDataIsCopied
    ld a, [de]
    ld [hli], a
    inc de
    dec bc
    ld a, b
    or c
  jr nz, .untilAllDataIsCopied
  ret

DisableLcd::
  ld a, [rLCDC]
  xor LCDCF_ON
  ld [rLCDC], a
  ret

EnableLcd::
  ld a, LCDCF_ON|LCDCF_BG8000|LCDCF_BG9800|LCDCF_BGON|LCDCF_OBJ8|LCDCF_OBJON
  ld [rLCDC], a
  ret

ClearGraphicsData::
  ; Tile memory
  ld hl, _VRAM
  ld bc, $800
  call ZeroMemory
  ; Tilemap
  ld hl, _SCRN0
  ld bc, 32*32
  call ZeroMemory
  ; Sprite attribute data
  ld hl, _OAMRAM
  ld bc, 40*4 ; 40 sprites, 4 bytes each
  call ZeroMemory
  ; Reset scroll registers
  ld a, 0
  ld [rSCX], a
  ld [rSCY], a
  ret

LoadGfx::
  ld hl, _VRAM
  ld de, TileData
  ld bc, EndSpriteData - TileData
  call CopyMemory
  ret

InitialiseRoom::
ROW = 0
; the room width does not match the gameboy's tilemap width
; so we can't just copy it over wholesale
REPT 18 ; the room is 18 tiles high
  ld hl, _SCRN0 + ROW * 32  ; native tilemap is 32 tiles wide
  ld de, MapData + ROW * 20 ; our room is 20 tiles wide
  ld bc, 20
  call CopyMemory
ROW = ROW + 1
ENDR
  ret

InitialiseSprite::
  ld a, 80
  ld [_OAMRAM], a
  ld a, 80
  ld [_OAMRAM+1], a
  ld a, 3
  ld [_OAMRAM+2], a
  ret

InitialiseMonochromePalettes:
  ld a, %11100100
  ld [rOBP0], a
  ld [rOBP1], a
  ld [rBGP], a
  ret

; todo: only do this on actual Colour hardware
InitialiseColourPalettes::
  ; Tile palette
  ld a, %10000000
  ld [rBCPS], a
BYTE_COUNTER = 0
REPT EndTileColourPaletteData - TileColourPaletteData
  ld a, [TileColourPaletteData + BYTE_COUNTER]
  ld [rBCPD], a
BYTE_COUNTER = BYTE_COUNTER + 1
ENDR
  ; Sprite palette
  ld a, %10000000
  ld [rOCPS], a
BYTE_COUNTER = 0
REPT EndSpritesColourPaletteData - SpritesColourPaletteData
  ld a, [SpritesColourPaletteData + BYTE_COUNTER]
  ld [rOCPD], a
BYTE_COUNTER = BYTE_COUNTER + 1
ENDR
  ret