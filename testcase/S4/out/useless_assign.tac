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
    %15 = "bcbcbcbcbc"
    parm %15
    call _PrintString
    %16 = "\n"
    parm %16
    call _PrintString
    parm 1
    parm 1
    %18 = call _Main.sum
    parm 1
    parm %18
    %20 = call _Main.sum
    parm 1
    parm %20
    %22 = call _Main.sum
    parm 1
    parm %22
    call _Main.sum
    parm %18
    call _PrintInt
    return
}

FUNC<_Main.sum> {
    %2 = (%0 + %1)
    return %2
}

