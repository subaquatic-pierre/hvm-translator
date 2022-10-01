$LCL_OFFSET //@i
D=A
@LCL
A=M+D // get LCL offset
M=0 // set LCL at address A value to 0
@SP
M=M+1
