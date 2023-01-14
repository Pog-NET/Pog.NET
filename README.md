<h1 align=center> Pog.NET
</h1>
<h3 align=center><b> A stack-based and register-based no dependency compiler toolchain written in rust!
</h3>
<br><br><p align="center">
 <img src="logo/vector/default-monochrome.svg" alt="drawing" width="200">
</p>
</b>

## Installation
### Install Pog.NET by running the `build.sh` script and the executables will be added to your linux /usr/bin/ folder
## Running programs
### You can run a program using `pognet run <program>` and you can turn it into an executable using `pognet deploy <program>`
## Hello world:

```
:println
    sout !
    cout #10
    ret
:main
    push " Hello, world! "
    jump :println
```
* Println
    * Pops the top of the stack and outptus it
    * Outputs ASCII 10 aka newline
    * Returns
* main
    * Pushes the string "Hello, world!" to the top of the stack
    * Jumps to the println function
### For more information check out [operations.md](./operations.md)
### For products using Pog.NET use [colors.md](./colors.md)