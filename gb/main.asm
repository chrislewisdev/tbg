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
SpriteData:
INCBIN "gen/sprite.2bpp"
EndSpriteData:
GbcPaletteData:
INCBIN "gen/sprite.pal"
EndGbcPaletteData:

SECTION "Game code", ROM0[$0150]
Startup:
  call WaitForNextVerticalBlank
  call DisableLcd
  call ClearGraphicsData
  call LoadSprites
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

LoadSprites::
  ld hl, _VRAM + 16 ; leave the first tile empty
  ld de, SpriteData
  ld bc, EndSpriteData - SpriteData
  call CopyMemory
  ret

InitialiseSprite::
  ld a, 80
  ld [_OAMRAM], a
  ld a, 80
  ld [_OAMRAM+1], a
  ld a, 1
  ld [_OAMRAM+2], a
  ret

InitialiseMonochromePalettes:
  ld a, %11100100
  ld [rOBP0], a
  ld [rOBP1], a
  ld [rBGP], a
  ret

; todo: better understand how this actually works
; todo: only do this on actual Colour hardware
InitialiseColourPalettes::
  ld a, %10000000
  ld [rOCPS], a
  ld de, GbcPaletteData
  ld bc, EndGbcPaletteData - GbcPaletteData
  .untilAllDataIsCopied
    ld a, [de]
    ld [rOCPD], a
    inc de
    dec bc
    ld a, b
    or c
  jr nz, .untilAllDataIsCopied
  ret