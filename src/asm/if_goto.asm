@SP
M=M-1
A=M
D=M // get value of last stack item
$LABEL // set A to address of label @label
D;JGT // if value is true 
D;JLT // if value is true
// if D=0, continue on normal flow, ie. value is false
