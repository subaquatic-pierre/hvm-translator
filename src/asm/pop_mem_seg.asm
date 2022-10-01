@SP
M=M-1 // retriet SP
A=M // set address to value of last stack address
D=M // set D to last value on stack
@R14 //@tmp
M=D // store value of stack in temp
$MEM_SEG_INDEX // @i
D=A
$MEM_SEG_LABEL // @LCL
D=M+D // get offset address
@R13 //@addr
M=D // store addr of memseg index
@R14 //@tmp
D=M // get value of stack from tmp 
@R13 //@addr
A=M
M=D // set address on mem seg @i to value

