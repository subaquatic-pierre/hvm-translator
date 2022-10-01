$ADDR_LBL // @Foo.3 | @R10
D=M // set D to value in mem address index
@SP
A=M
M=D // set current SP address to D
@SP
M=M+1 // increment stack pointer