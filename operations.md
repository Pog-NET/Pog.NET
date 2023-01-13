# All Pog.NET operations

## Type Annotations:
* \# = Number (example: `#10`)
* \$ = Register (exampe: `$r0`)
* \: = Label (example: `:label`)
* \" = String (example: `" Test "`)
* \! = Last item of stack (example: `push !`)
* \! = Second to last item of stack (example: `push !!`)
## Registers:
The avaliable registers are:
* r0
* r1
* r2
* r3
* r4
* r5
* r6
* r7
* r8
* r9
## Operations:
### Push - Pushes a type to the top of the stack.
Example:
```
:main
    push #30
    push " Test "
    sout !
    nout !
```
Output: Test30
### Pop - Pops a type into a register
Example:
```
:main
    push #30
    pop $r0
    nout $r0
```
Output: 30
### Add - Adds the top two stack values
Example:
```
:main
    push #30
    push #40
    add
    nout !
```
Output: 70
### Sub - Subtracts the top two stack values
Example:
```
:main
    push #30
    push #40
    sub
    nout !
```
Output: 10
### Mul - Multiplies the top two stack values
Example:
```
:main
    push #10
    push #10
    mul
    nout !
```
Output: 100
### Nout - Prints a number
Example:
```
:main
    nout #30
```
Output: 30
### Sout - Prints a string
Example:
```
:main
    sout " Hello, world! "
```
Output: Hello, world!
### Cout - Prints a character
Example:
```
:main
    cout #65
```
Output: A
### Jump - Jumpts to a label
Example:
```
:test
    sout " Hello from the test label! "
    ret
:main
    jump test
```
Output: Hello from the test label!
### Je - Jumpts to a label if the top two stack values are equal
Example:
```
:test
    sout " Hello from the test label! "
    ret
:main
    push #30
    push #30
    je test
```
Output: Hello from the test label!
<br><br>Example:
```
:test
    sout " Hello from the test label! "
    ret
:main
    push #30
    push #20
    je test
```
Output: (None)

### Jne - Jumpts to a label if the top two stack values are not equal
Example:
```
:test
    sout " Hello from the test label! "
    ret
:main
    push #30
    push #30
    jne test
```
Output: (None)
<br><br>Example:
```
:test
    sout " Hello from the test label! "
    ret
:main
    push #30
    push #20
    jne test
```
Output: Hello from the test label!

### Ret - returns from a label
Example:
```
:test
    sout " Hello! "
    ret
    sout " Hello! "
:main
    jump test
```
Output: Hello!