mov 25 bx
mov 2 cx

.prime_loop
  div bx cx ax
  cnd ax 0
  jne .end
  add cx 1 cx
  cnd cx bx
  jlt .prime_loop

.end
