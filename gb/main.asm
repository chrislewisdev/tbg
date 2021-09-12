INCLUDE "hardware.inc"

SECTION "ROM Title", ROM0[$0134]
  DB "tbg"

SECTION "Nintendo Logo", ROM0[$0104]
  NINTENDO_LOGO

SECTION "Entrypoint", ROM0[$0100]
  nop
  jp Startup

SECTION "Gameboy Colour Support", ROM0[$0143]
  DB CART_COMPATIBLE_DMG_GBC

SECTION "Vertical blank handler", ROM0[$0040]
  call VerticalBlankHandler
  reti

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

SECTION "Variables", WRAM0
isGbc: db
verticalBlankFlag: db
input: db
previousInput: db
unit:
  .sx db
  .sy db
  .ctrl_hi db
  .ctrl_lo db

SPR0_Y  EQU _OAMRAM
SPR0_X  EQU _OAMRAM+1
SPR0_ID EQU _OAMRAM+2

BTN_DOWN      EQU %10000000
BTN_UP        EQU %01000000
BTN_LEFT      EQU %00100000
BTN_RIGHT     EQU %00010000
BTN_START     EQU %00001000
BTN_SELECT    EQU %00000100
BTN_B         EQU %00000010
BTN_A         EQU %00000001

CallHlFunctionPointer: MACRO
  ld bc, @ + 5
  push bc
  jp hl
ENDM

SECTION "Game code", ROM0[$0150]
Startup:
  di
  call CheckForGbc
  call WaitForNextVerticalBlank
  call DisableLcd
  call ClearGraphicsData
  call LoadGfx
  call InitialiseRoom
  call InitialiseMonochromePalettes
  call InitialiseColourPalettes
  call InitialisePlayer
  call EnableLcd
  call ConfigureInterrupts
GameLoop:
  call ReadInput
  call ExploreState_Update
  call WaitForNextVerticalBlankViaInterrupt
  call ExploreState_Draw
  jr GameLoop

ExploreState_Update:
  call GetCmdForUnit
  call ExecuteCmdIfPresentInHl
  ret

GetCmdForUnit:
  ld a, [unit.ctrl_hi]
  ld h, a
  ld a, [unit.ctrl_lo]
  ld l, a
  CallHlFunctionPointer
  ret

ExecuteCmdIfPresentInHl:
  ld a, h
  and a
  ret z
  CallHlFunctionPointer
  ret

ExploreState_Draw:
  call DrawPlayer
  ret

PlayerController:
  ld a, [input]
  ld b, a
  and BTN_LEFT
  jr nz, .left
  ld a, b
  and BTN_RIGHT
  jr nz, .right
  ld a, b
  and BTN_UP
  jr nz, .up
  ld a, b
  and BTN_DOWN
  jr nz, .down
  ld hl, $0000
  ret
  .left
    ld hl, MoveLeft
    ret
  .right
    ld hl, MoveRight
    ret
  .up
    ld hl, MoveUp
    ret
  .down
    ld hl, MoveDown
    ret

; todo: make commands indexable
MoveLeft:
  ld a, [unit.sx]
  dec a
  ld [unit.sx], a
  ret
MoveRight:
  ld a, [unit.sx]
  inc a
  ld [unit.sx], a
  ret
MoveUp:
  ld a, [unit.sy]
  dec a
  ld [unit.sy], a
  ret
MoveDown:
  ld a, [unit.sy]
  inc a
  ld [unit.sy], a
  ret

SECTION "Functions", ROM0
VerticalBlankHandler:
  push af
  ld a, 1
  ld [verticalBlankFlag], a
  pop af
  ret

; Waits for the START of a new vblank period to ensure maximum time is available.
WaitForNextVerticalBlankViaInterrupt:
  .untilVerticalBlank
    halt
    ld a, [verticalBlankFlag]
    or a
    jr z, .untilVerticalBlank
  ld a, 0
  ld [verticalBlankFlag], a
  ret

WaitForNextVerticalBlank:
  .untilVerticalBlank
    ld a, [rLY]
    cp 144
  jr nz, .untilVerticalBlank
  ret

; hl = destination address
; bc = no. bytes to zero
ZeroMemory:
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
CopyMemory:
  .untilAllDataIsCopied
    ld a, [de]
    ld [hli], a
    inc de
    dec bc
    ld a, b
    or c
  jr nz, .untilAllDataIsCopied
  ret

DisableLcd:
  ld a, [rLCDC]
  xor LCDCF_ON
  ld [rLCDC], a
  ret

EnableLcd:
  ld a, LCDCF_ON|LCDCF_BG8000|LCDCF_BG9800|LCDCF_BGON|LCDCF_OBJ8|LCDCF_OBJON
  ld [rLCDC], a
  ret

ConfigureInterrupts:
  ld a, IEF_VBLANK
  ld [rIE], a
  ei
  ret

ClearGraphicsData:
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

InitialiseRoom:
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

InitialiseMonochromePalettes:
  ld a, %11100100
  ld [rOBP0], a
  ld [rOBP1], a
  ld [rBGP], a
  ret

InitialiseColourPalettes:
  ; only do this on actual GBC hardware
  ld a, [isGbc]
  cp 1
  ret nz
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

; only works when run IMMEDIATELY on startup
CheckForGbc:
  cp $11
  jr nz, .isNotGbc
  .isGbc
    ld a, 1
    ld [isGbc], a
    ret
  .isNotGbc
    ld a, 0
    ld [isGbc], a
    ret

ReadInput:
  ld a, [input]
  ld [previousInput], a
  ld a, P1F_GET_DPAD
  ld [rP1], a
  ld a, [rP1]
  ld a, [rP1]
  ld a, [rP1]
  ld a, [rP1]
  and a, %1111
  swap a
  ld b, a
  ld a, P1F_GET_BTN
  ld [rP1], a
  ld a, [rP1]
  ld a, [rP1]
  ld a, [rP1]
  ld a, [rP1]
  and a, %1111
  or a, b
  cpl
  ld [input], a
  ret

InitialisePlayer:
  ld a, 80
  ld [unit.sx], a
  ld a, 80
  ld [unit.sy], a
  ld bc, PlayerController
  ld a, b
  ld [unit.ctrl_hi], a
  ld a, c
  ld [unit.ctrl_lo], a
  ret

DrawPlayer:
  ld a, [unit.sx]
  ld [SPR0_X], a
  ld a, [unit.sy]
  ld [SPR0_Y], a
  ld a, 3
  ld [SPR0_ID], a
  ret
