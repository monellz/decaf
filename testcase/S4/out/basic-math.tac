VTBL<_Maths> {
    0
    "Maths"
}

VTBL<_Main> {
    0
    "Main"
}

FUNC<_Maths._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Maths>
    *(%0 + 0) = %1
    return %0
}

FUNC<_Main._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Main>
    *(%0 + 0) = %1
    return %0
}

FUNC<_Maths.abs> {
    %1 = (%0 >= 0)
    if (%1 == 0) branch %2
    return %0
    %2:
    %2 = - %0
    return %2
}

FUNC<_Maths.pow> {
    %2 = 1
    %3 = 0
    branch %2
    %1:
    %4 = (%2 * %0)
    %2 = %4
    %5 = (%3 + 1)
    %3 = %5
    %2:
    %6 = (%3 < %1)
    if (%6 != 0) branch %1
    return %2
}

FUNC<_Maths.log> {
    %1 = (%0 < 1)
    if (%1 == 0) branch %2
    return -1
    %2:
    %3 = 0
    branch %4
    %3:
    %4 = (%3 + 1)
    %3 = %4
    %5 = (%0 / 2)
    %0 = %5
    %4:
    %6 = (%0 > 1)
    if (%6 != 0) branch %3
    return %3
}

FUNC<_Maths.max> {
    %2 = (%0 > %1)
    if (%2 == 0) branch %2
    return %0
    %2:
    return %1
}

FUNC<_Maths.min> {
    %2 = (%0 < %1)
    if (%2 == 0) branch %2
    return %0
    %2:
    return %1
}

FUNC<main> {
    parm -1
    %0 = call _Maths.abs
    parm %0
    call _PrintInt
    %2 = "\n"
    parm %2
    call _PrintString
    parm 2
    parm 3
    %3 = call _Maths.pow
    parm %3
    call _PrintInt
    %4 = "\n"
    parm %4
    call _PrintString
    parm 16
    %5 = call _Maths.log
    parm %5
    call _PrintInt
    %6 = "\n"
    parm %6
    call _PrintString
    parm 1
    parm 2
    %7 = call _Maths.max
    parm %7
    call _PrintInt
    %8 = "\n"
    parm %8
    call _PrintString
    parm 1
    parm 2
    %9 = call _Maths.min
    parm %9
    call _PrintInt
    %10 = "\n"
    parm %10
    call _PrintString
    return
}

