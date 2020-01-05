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
    %2 = call _Alloc
    %1 = FUNC<_lambda_9_15>
    *(%2 + 0) = %1
    *(%2 + 4) = %0
    *(%0 + 4) = %2
    %4 = *(%0 + 4)
    parm 8
    %7 = call _Alloc
    %6 = FUNC<_lambda_10_28>
    *(%7 + 0) = %6
    *(%7 + 4) = %0
    %5 = *(%4 + 0)
    parm %4
    parm %7
    %8 = call %5
    %3 = %8
    %9 = *(%3 + 0)
    parm %3
    parm 10
    %10 = call %9
    parm %10
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
    %2 = *(%0 + 8)
    %3 = *(%0 + 4)
    %4 = *(%3 + 4)
    %5 = *(%4 + 0)
    parm %4
    parm %2
    %6 = call %5
    %7 = *(%6 + 0)
    parm %6
    parm %1
    %8 = call %7
    return %8
}

FUNC<_lambda_9_15> {
    parm 12
    %4 = call _Alloc
    %3 = FUNC<_lambda_9_46>
    *(%4 + 0) = %3
    %5 = *(%0 + 4)
    *(%4 + 4) = %5
    *(%4 + 8) = %1
    %2 = *(%1 + 0)
    parm %1
    parm %4
    %6 = call %2
    return %6
}

FUNC<_lambda_10_47> {
    %2 = *(%0 + 8)
    %3 = (%1 == 0)
    if (%3 == 0) branch %0
    return 1
    branch %1
    %0:
    %5 = (%1 - 1)
    %4 = *(%2 + 0)
    parm %2
    parm %5
    %6 = call %4
    %7 = (%1 * %6)
    return %7
    %1:
}

FUNC<_lambda_10_28> {
    parm 12
    %3 = call _Alloc
    %2 = FUNC<_lambda_10_47>
    *(%3 + 0) = %2
    %4 = *(%0 + 4)
    *(%3 + 4) = %4
    *(%3 + 8) = %1
    return %3
}

