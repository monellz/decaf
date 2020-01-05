VTBL<_Main> {
    0
    "Main"
    FUNC<_Main.foo>
}

FUNC<_Main._new> {
    parm 8
    %0 = call _Alloc
    %1 = VTBL<_Main>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    return %0
}

FUNC<main> {
    %1 = call _Main._new
    parm 8
    %4 = call _Alloc
    %0 = FUNC<_foo_3_20>
    *(%4 + 0) = %0
    *(%4 + 4) = %1
    %5 = *(%4 + 0)
    parm %4
    call %5
    return
}

FUNC<_Main.foo> {
    parm 8
    %6 = call _Alloc
    %5 = FUNC<_lambda_9_15>
    *(%6 + 0) = %5
    *(%6 + 4) = %0
    *(%0 + 4) = %6
    %4 = *(%0 + 4)
    parm 8
    %3 = call _Alloc
    %2 = FUNC<_lambda_10_28>
    *(%3 + 0) = %2
    *(%3 + 4) = %0
    %5 = *(%4 + 0)
    parm %4
    parm %3
    %6 = call %5
    %2 = %6
    %8 = *(%2 + 0)
    parm %2
    parm 10
    %9 = call %8
    parm %9
    call _PrintInt
    return
}

FUNC<_foo_3_20> {
    %2 = *(%0 + 4)
    parm %2
    %3 = *(%2 + 0)
    %3 = *(%3 + 8)
    call %3
    return
}

FUNC<_lambda_9_46> {
    %3 = *(%0 + 4)
    %4 = *(%3 + 4)
    %6 = *(%0 + 8)
    %5 = *(%4 + 0)
    parm %4
    parm %6
    %7 = call %5
    %8 = *(%7 + 0)
    parm %7
    parm %1
    %10 = call %8
    return %10
}

FUNC<_lambda_9_15> {
    parm 12
    %12 = call _Alloc
    %11 = FUNC<_lambda_9_46>
    *(%12 + 0) = %11
    *(%12 + 4) = %0
    *(%12 + 8) = %1
    %3 = *(%1 + 0)
    parm %1
    parm %12
    %4 = call %3
    return %4
}

FUNC<_lambda_10_47> {
    %3 = (%1 == 0)
    if (%3 == 0) branch %0
    return 1
    branch %1
    %0:
    %5 = *(%0 + 8)
    %8 = (%1 - 1)
    %6 = *(%5 + 0)
    parm %5
    parm %8
    %9 = call %6
    %10 = (%1 * %9)
    return %10
    %1:
}

FUNC<_lambda_10_28> {
    parm 12
    %12 = call _Alloc
    %11 = FUNC<_lambda_10_47>
    *(%12 + 0) = %11
    *(%12 + 4) = %0
    *(%12 + 8) = %1
    return %12
}

