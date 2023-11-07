mov 14 ax

mov 1 bx
mov 0 cx

.loop
  cnd ax 0
  jeq .end

  add cx bx dx
  mov bx cx
  mov dx bx
  
  sub ax 1 ax
  jmp .loop

.end
