@SP
M=M-1 // get last value on stack
A=M
D=M // set D to last value on stack
@SP 
M=M-1 // get second last value on stack
A=M
D=M-D // compare two values, M=top(second last value) - D=bottom(last value on stack) eg. 5-6
@$TRUE_LABEL // jump to true if condition is true for compare command
$COMPARE_COMMAND // D;JEQ | D;GLT | ...
@SP
A=M
M=0 // set stack value to false
@$END_LABEL
0;JEQ // jmp to end, skip set true commands
($TRUE_LABEL)
@SP
A=M
M=-1 // set stack value to true
($END_LABEL)
@SP
M=M+1 // increment SP