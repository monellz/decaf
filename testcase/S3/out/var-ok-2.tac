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
    %3 = "zjm1926"
    %2 = %3
    %5 = "glgjssy qyhfbqz"
    %4 = %5
    %7 = "xgjz"
    %6 = %7
    %8 = 0
    %9 = %0
    branch %0
    %1:
    %10 = (%8 + %9)
    %8 = %10
    %11 = (%9 + 1)
    %9 = %11
    %0:
    %12 = (%9 < %1)
    if (%12 != 0) branch %1
    %2:
    return %8
}

FUNC<main> {
    %0 = 1
    %1 = 10
    parm 4
    %4 = call _Alloc
    %2 = FUNC<_f_14_15>
    *(%4 + 0) = %2
    %5 = *(%4 + 0)
    parm %4
    parm %0
    parm %1
    %6 = call %5
    parm %6
    call _PrintInt
    return
}

FUNC<_f_14_15> {
    parm %1
    parm %2
    %3 = call _Main.f
    return %3
}

