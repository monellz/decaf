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

FUNC<_Main.f> {
    %1 = (%0 + 2)
    return %1
}

FUNC<_Main.genadd> {
    parm 4
    %2 = call _Alloc
    %0 = FUNC<_f_6_16>
    *(%2 + 0) = %0
    return %2
}

FUNC<main> {
    parm 4
    %3 = call _Alloc
    %1 = FUNC<_genadd_10_17>
    *(%3 + 0) = %1
    %4 = *(%3 + 0)
    parm %3
    %5 = call %4
    %0 = %5
    parm 4
    %8 = call _Alloc
    %6 = FUNC<_f_11_15>
    *(%8 + 0) = %6
    %9 = *(%8 + 0)
    parm %8
    parm 2
    %10 = call %9
    parm %10
    call _PrintInt
    parm 4
    %13 = call _Alloc
    %11 = FUNC<_f_12_15>
    *(%13 + 0) = %11
    %14 = *(%13 + 0)
    parm %13
    parm 4
    %15 = call %14
    parm %15
    call _PrintInt
    return
}

FUNC<_f_6_16> {
    parm %1
    %1 = call _Main.f
    return %1
}

FUNC<_genadd_10_17> {
    %2 = call _Main.genadd
    return %2
}

FUNC<_f_11_15> {
    parm %1
    %7 = call _Main.f
    return %7
}

FUNC<_f_12_15> {
    parm %1
    %12 = call _Main.f
    return %12
}

