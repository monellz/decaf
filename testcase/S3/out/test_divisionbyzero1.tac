VTBL<_Main> {
    0
    "Main"
}

FUNC<_Main._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Main>
    *(%0 + 0) = %1
    return %0
}

FUNC<main> {
    %0 = 0
    %1 = "testing division by 0 runtime error1\n"
    parm %1
    call _PrintString
    %4 = (0 != 0)
    if (%4 == 0) branch %0
    %3 = 0
    %2 = (13 / %3)
    branch %1
    %0:
    %5 = "Decaf runtime error: Division by zero error\n"
    parm %5
    call _PrintString
    call _Halt
    %1:
    %0 = %2
    %6 = "end"
    parm %6
    call _PrintString
    return
}

