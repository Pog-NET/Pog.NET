:println
    sout $r0
    cout #10
    ret
:main
    push " Hello, world! "
    pop $r0
    jump :println