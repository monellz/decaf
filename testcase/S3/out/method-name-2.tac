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
    parm 4
    %1 = call _Alloc
    %0 = FUNC<_double_3_9>
    *(%1 + 0) = %0
    parm 4
    %4 = call _Alloc
    %3 = FUNC<_print_3_16>
    *(%4 + 0) = %3
    %2 = *(%1 + 0)
    parm %1
    parm %4
    call %2
    parm 4
    %6 = call _Alloc
    %5 = FUNC<_filter_4_9>
    *(%6 + 0) = %5
    parm 4
    %9 = call _Alloc
    %8 = FUNC<_printi_4_16>
    *(%9 + 0) = %8
    parm 4
    %12 = call _Alloc
    %10 = FUNC<_even_4_24>
    *(%12 + 0) = %10
    %7 = *(%6 + 0)
    parm %6
    parm %9
    parm %12
    parm 10
    call %7
    return
}

FUNC<_Main.print> {
    %0 = "hello\n"
    parm %0
    call _PrintString
    return
}

FUNC<_Main.printi> {
    parm %0
    call _PrintInt
    %1 = " "
    parm %1
    call _PrintString
    return
}

FUNC<_Main.double> {
    %1 = *(%0 + 0)
    parm %0
    call %1
    %2 = *(%0 + 0)
    parm %0
    call %2
    return
}

FUNC<_Main.even> {
    %3 = (2 != 0)
    if (%3 == 0) branch %0
    %2 = 2
    %1 = (%0 % %2)
    branch %1
    %0:
    %4 = "Decaf runtime error: Division by zero error\n"
    parm %4
    call _PrintString
    call _Halt
    %1:
    %5 = (%1 == 0)
    return %5
}

FUNC<_Main.filter> {
    %3 = 0
    branch %0
    %1:
    %4 = *(%1 + 0)
    parm %1
    parm %3
    %5 = call %4
    if (%5 == 0) branch %3
    %6 = *(%0 + 0)
    parm %0
    parm %3
    call %6
    %3:
    %7 = (%3 + 1)
    %3 = %7
    %0:
    %8 = (%3 < %2)
    if (%8 != 0) branch %1
    %2:
    return
}

FUNC<_double_3_9> {
    parm %1
    call _Main.double
    return
}

FUNC<_print_3_16> {
    call _Main.print
    return
}

FUNC<_filter_4_9> {
    parm %1
    parm %2
    parm %3
    call _Main.filter
    return
}

FUNC<_printi_4_16> {
    parm %1
    call _Main.printi
    return
}

FUNC<_even_4_24> {
    parm %1
    %11 = call _Main.even
    return %11
}

