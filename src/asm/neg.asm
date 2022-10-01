@SP
M=M-1 // get last value on stack
A=M
D=M // set D to last value on stack
$NEG_COMMAND // M=!D | M=–D
@SP
M=M+1