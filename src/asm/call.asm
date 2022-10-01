$RET_ADD // push retAddLabel @Foo$ret.1
D=A
@SP
A=M
M=D // push return label address to stack
@SP
M=M+1 // inc SP, SP+=1
@LCL // push LCL
D=M
@SP
A=M
M=D // push LCL onto stack
@SP
M=M+1 // inc SP, SP+=1
@ARG // push ARG
D=M
@SP
A=M
M=D // push ARG onto stack
@SP
M=M+1 // inc SP, SP+=1
@THIS // push THIS
D=M
@SP
A=M
M=D // push THIS onto stack
@SP
M=M+1 // inc SP, SP+=1
@THAT // push THAT
D=M
@SP
A=M
M=D // push THAT onto stack
@SP
M=M+1 // inc SP, SP+=1
$ARG_OFFSET // @i
D=A
@SP
D=M-D // D = SP value minus offset
@5
D=D-A // reduce D by const 5
@ARG
M=D // reposition ARG
@SP
D=M
@LCL
M=D // reposition LCL
$FUCNTION_NAME // @Foo.mult GOTO function
0;JMP
$SET_LABEL // (Foo$ret.1)

