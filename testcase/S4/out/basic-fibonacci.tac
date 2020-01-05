VTBL<_Main> {
    0
    "Main"
}

VTBL<_Fibonacci> {
    0
    "Fibonacci"
    FUNC<_Fibonacci.get>
}

FUNC<_Main._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Main>
    *(%0 + 0) = %1
    return %0
}

FUNC<_Fibonacci._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Fibonacci>
    *(%0 + 0) = %1
    return %0
}

FUNC<main> {
    %0 = 0
    %2 = call _Fibonacci._new
    branch %2
    %1:
    parm %2
    parm %0
    %4 = *(%2 + 0)
    %4 = *(%4 + 8)
    %3 = call %4
    parm %3
    call _PrintInt
    %5 = "\n"
    parm %5
    call _PrintString
    %6 = (%0 + 1)
    %0 = %6
    %2:
    %7 = (%0 < 10)
    if (%7 != 0) branch %1
    return
}

FUNC<_Fibonacci.get> {
    %2 = (%1 < 2)
    if (%2 == 0) branch %2
    return 1
    %2:
    %4 = (%1 - 1)
    parm %0
    parm %4
    %10 = *(%0 + 0)
    %11 = *(%10 + 8)
    %3 = call %11
    %7 = (%1 - 2)
    parm %0
    parm %7
    %6 = call %11
    %9 = (%3 + %6)
    return %9
}

