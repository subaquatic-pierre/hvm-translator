// POP to return
@SP
M=M-1 // get last value on stack address 
A=M
D=M // store last value on stack in D
@ARG 
A=M
M=D // set ARG[0] to popped stack value

// Reposition SP ARG
@ARG
D=M
@SP // SP = ARG + 1, repositions SP of the caller
M=D+1 // repositions SP of the caller

// ENDFRAME ADDR
@LCL
D=M // get mem address of LCL
@R13 // endframe
M=D // endframe tmp var

// RETURN ADDR
@5 // gets the return address
D=A
@R13
D=M-D
A=D
D=M
@R14 // retAddr
M=D // set retAddr tmp 

// LCL
@4 // restores LCL of the caller
D=A
@R13
A=M-D
D=M
@LCL
M=D // LCL = *(endFrame – 4)

// ARG
@3 // restores ARG of the caller
D=A
@R13
A=M-D
D=M
@ARG 
M=D // ARG = *(endFrame – 3)

// THIS
@2 // restores THIS of the caller
D=A
@R13
A=M-D
D=M
@THIS
M=D // THIS = *(endFrame – 2)

// THAT
@R13
A=M-1
D=M
@THAT // restores THAT of the caller
M=D // THAT = *(endFrame – 1)

@R14
A=M
0;JMP // jump to return address