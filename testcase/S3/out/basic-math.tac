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
    if (%1 == 0) branch %0
    return %0
    branch %1
    %0:
    %2 = - %0
    return %2
    %1:
}

FUNC<_Maths.pow> {
    %2 = 1
    %3 = 0
    branch %0
    %1:
    %4 = (%2 * %0)
    %2 = %4
    %5 = (%3 + 1)
    %3 = %5
    %0:
    %6 = (%3 < %1)
    if (%6 != 0) branch %1
    %2:
    return %2
}

FUNC<_Maths.log> {
    %1 = (%0 < 1)
    if (%1 == 0) branch %0
    %2 = - 1
    return %2
    %0:
    %3 = 0
    branch %1
    %2:
    %4 = (%3 + 1)
    %3 = %4
    %7 = (2 != 0)
    if (%7 == 0) branch %4
    %6 = 2
    %5 = (%0 / %6)
    branch %5
    %4:
    %8 = "Decaf runtime error: Division by zero error\n"
    parm %8
    call _PrintString
    call _Halt
    %5:
    %0 = %5
    %1:
    %9 = (%0 > 1)
    if (%9 != 0) branch %2
    %3:
    return %3
}

FUNC<_Maths.max> {
    %2 = (%0 > %1)
    if (%2 == 0) branch %0
    return %0
    branch %1
    %0:
    return %1
    %1:
}

FUNC<_Maths.min> {
    %2 = (%0 < %1)
    if (%2 == 0) branch %0
    return %0
    branch %1
    %0:
    return %1
    %1:
}

FUNC<main> {
    parm 4
    %2 = call _Alloc
    %0 = FUNC<_abs_49_21>
    *(%2 + 0) = %0
    %4 = - 1
    %3 = *(%2 + 0)
    parm %2
    parm %4
    %5 = call %3
    parm %5
    call _PrintInt
    %6 = "\n"
    parm %6
    call _PrintString
    parm 4
    %9 = call _Alloc
    %7 = FUNC<_pow_50_21>
    *(%9 + 0) = %7
    %10 = *(%9 + 0)
    parm %9
    parm 2
    parm 3
    %11 = call %10
    parm %11
    call _PrintInt
    %12 = "\n"
    parm %12
    call _PrintString
    parm 4
    %15 = call _Alloc
    %13 = FUNC<_log_51_21>
    *(%15 + 0) = %13
    %16 = *(%15 + 0)
    parm %15
    parm 16
    %17 = call %16
    parm %17
    call _PrintInt
    %18 = "\n"
    parm %18
    call _PrintString
    parm 4
    %21 = call _Alloc
    %19 = FUNC<_max_52_21>
    *(%21 + 0) = %19
    %22 = *(%21 + 0)
    parm %21
    parm 1
    parm 2
    %23 = call %22
    parm %23
    call _PrintInt
    %24 = "\n"
    parm %24
    call _PrintString
    parm 4
    %27 = call _Alloc
    %25 = FUNC<_min_53_21>
    *(%27 + 0) = %25
    %28 = *(%27 + 0)
    parm %27
    parm 1
    parm 2
    %29 = call %28
    parm %29
    call _PrintInt
    %30 = "\n"
    parm %30
    call _PrintString
    return
}

FUNC<_abs_49_21> {
    parm %1
    %1 = call _Maths.abs
    return %1
}

FUNC<_pow_50_21> {
    parm %1
    parm %2
    %8 = call _Maths.pow
    return %8
}

FUNC<_log_51_21> {
    parm %1
    %14 = call _Maths.log
    return %14
}

FUNC<_max_52_21> {
    parm %1
    parm %2
    %20 = call _Maths.max
    return %20
}

FUNC<_min_53_21> {
    parm %1
    parm %2
    %26 = call _Maths.min
    return %26
}

