$OFFSET_INDEX // @i
D=A // asign offset to D
$MEM_SEG_LABEL // @LCL
A=D+M // offset address
D=M // set D to value in mem address index
@SP
A=M
M=D // set current SP address > val to D
@SP
M=M+1 // increment stack pointer