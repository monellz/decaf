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
    %1 = %2
    branch %0
    %1:
    parm 8
    %7 = call _Alloc
    %3 = FUNC<_get_11_22>
    *(%7 + 0) = %3
    *(%7 + 4) = %1
    %8 = *(%7 + 0)
    parm %7
    parm %0
    %9 = call %8
    parm %9
    call _PrintInt
    %10 = "\n"
    parm %10
    call _PrintString
    %11 = (%0 + 1)
    %0 = %11
    %0:
    %12 = (%0 < 10)
    if (%12 != 0) branch %1
    %2:
    return
}

FUNC<_Fibonacci.get> {
    %2 = (%1 < 2)
    if (%2 == 0) branch %0
    return 1
    %0:
    parm 8
    %7 = call _Alloc
    %3 = FUNC<_get_24_16>
    *(%7 + 0) = %3
    *(%7 + 4) = %0
    %9 = (%1 - 1)
    %8 = *(%7 + 0)
    parm %7
    parm %9
    %10 = call %8
    parm 8
    %15 = call _Alloc
    %11 = FUNC<_get_24_33>
    *(%15 + 0) = %11
    *(%15 + 4) = %0
    %17 = (%1 - 2)
    %16 = *(%15 + 0)
    parm %15
    parm %17
    %18 = call %16
    %19 = (%10 + %18)
    return %19
}

FUNC<_get_11_22> {
    %5 = *(%0 + 4)
    parm %5
    parm %1
    %6 = *(%5 + 0)
    %6 = *(%6 + 8)
    %4 = call %6
    return %4
}

FUNC<_get_24_16> {
    %5 = *(%0 + 4)
    parm %5
    parm %1
    %6 = *(%5 + 0)
    %6 = *(%6 + 8)
    %4 = call %6
    return %4
}

FUNC<_get_24_33> {
    %13 = *(%0 + 4)
    parm %13
    parm %1
    %14 = *(%13 + 0)
    %14 = *(%14 + 8)
    %12 = call %14
    return %12
}

