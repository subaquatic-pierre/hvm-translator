@$CONST_VAL // assigned at runtime
D=A // asign const to D
@SP
A=M
M=D // assign current RAM[SP] to D
@SP
M=M+1 // increment SP