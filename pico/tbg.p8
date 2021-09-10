pico-8 cartridge // http://www.pico-8.com
version 32
__lua__
--pico
function _init()
 palt(0,false)
 palt(14,true)
 state={
  update=update_game,
  draw=draw_game
 }
 globalstate={
  player={
   x=3,
   y=3,
   sx=3*8,
   sy=3*8,
   getcmd=player_control,
  }
 }
end

function update_game(gs)
 if(cmd==nil)then
  cmd=gs.player.getcmd(gs.player)
 end
 if(cmd!=nil)then
  done=cmd(gs.player)
  if(done)cmd=nil
 end
end

function draw_game(gs)
 cls()
 map(0,0,0,0,16,16)
 spr(1,gs.player.sx,gs.player.sy)
end

function _update60()
 state.update(globalstate)
end

function _draw()
 state.draw(globalstate)
end
-->8
--core

function sign(x)
 if(x==0)return 0
 return sgn(x)
end

function solid(x,y)
 return mget(x,y)!=3
end
-->8
--player

function player_control(p)
 if(btnp(0) and not solid(p.x-1,p.y))then
  return moveleft
 end
 if(btnp(1) and not solid(p.x+1,p.y))then
  return moveright
 end
 if(btnp(2) and not solid(p.x,p.y-1))then
  return moveup
 end
 if(btnp(3) and not solid(p.x,p.y+1))then
  return movedown
 end
 
 return nil
end

-->8
--cmds

function generic_move(u,dx,dy)
 local tx=(u.x+dx)*8
 local ty=(u.y+dy)*8
 
 u.sx+=2*sign(dx)
 u.sy+=2*sign(dy)
 
 if(u.sx==tx and u.sy==ty)then
  u.x+=dx
  u.y+=dy
  return true
 end
 
 return false
end

function moveleft(u)
 return generic_move(u,-1,0)
end
function moveright(u)
 return generic_move(u,1,0)
end
function moveup(u)
 return generic_move(u,0,-1)
end
function movedown(u)
 return generic_move(u,0,1)
end

__gfx__
00000000e0eeee0e00000000dddddddd151515150000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000f0000f000000000dddd7ddd111111110000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
007007000f33333000000000dddddddd515551510000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000770000f0fff0000000000ddddddd7111111110000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000770000ffffff000000000dddddddd151555150000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0070070000000000000000007ddd7ddd111111110000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000f8888f000000000dddddddd555151510000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000e000000e00000000ddddd7dd000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
__map__
0202020202020202020202020202020202000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0204040404040402040404040404020202000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0203030303030302030303030303020202000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0203030303030302030303030303020202000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0203030303030302030303030303020202000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0203030303030302030303030303020200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0203030303030304030303030303040200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0203030303030303030303030303030200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0203030303030303030303030303030200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0202020202020202020202030303020200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0202020202020202020202030303020200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0202020202020202020202030303020200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0202020202020202020202030303020202000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0202020202020202020202030303020202000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0202020202020202020202030303020202000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0202020202020202020202020202020202000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000