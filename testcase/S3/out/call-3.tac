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

FUNC<_Main.add1> {
    %1 = (24 + %0)
    return %1
}

FUNC<_Main.add2> {
    %1 = (26 + %0)
    return %1
}

FUNC<_Main.genadd> {
    %1 = (%0 < 25)
    if (%1 == 0) branch %0
    parm 4
    %4 = call _Alloc
    %2 = FUNC<_add1_12_14>
    *(%4 + 0) = %2
    return %4
    branch %1
    %0:
    parm 4
    %7 = call _Alloc
    %5 = FUNC<_add2_14_17>
    *(%7 + 0) = %5
    return %7
    %1:
}

FUNC<main> {
    parm 8
    %2 = call _Alloc
    %1 = FUNC<_lambda_18_17>
    *(%2 + 0) = %1
    *(%2 + 4) = %0
    %0 = %2
    %3 = *(%0 + 0)
    parm %0
    parm 24
    %4 = call %3
    %5 = *(%4 + 0)
    parm %4
    parm 24
    %6 = call %5
    parm %6
    call _PrintInt
    %7 = *(%0 + 0)
    parm %0
    parm 26
    %8 = call %7
    %9 = *(%8 + 0)
    parm %8
    parm 26
    %10 = call %9
    parm %10
    call _PrintInt
    return
}

FUNC<_add1_12_14> {
    parm %1
    %3 = call _Main.add1
    return %3
}

FUNC<_add2_14_17> {
    parm %1
    %6 = call _Main.add2
    return %6
}

FUNC<_genadd_18_32> {
    parm %1
    %3 = call _Main.genadd
    return %3
}

FUNC<_lambda_18_17> {
    parm 4
    %4 = call _Alloc
    %2 = FUNC<_genadd_18_32>
    *(%4 + 0) = %2
    %5 = *(%4 + 0)
    parm %4
    parm %1
    %6 = call %5
    return %6
}

