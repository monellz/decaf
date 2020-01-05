VTBL<_Main> {
    0
    "Main"
    FUNC<_Main.niam>
}

FUNC<_Main._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Main>
    *(%0 + 0) = %1
    return %0
}

FUNC<_Main.niam> {
    %1 = 0
    parm 8
    %4 = call _Alloc
    %3 = FUNC<_lambda_7_17>
    *(%4 + 0) = %3
    *(%4 + 4) = %0
    %2 = %4
    parm 16
    %7 = call _Alloc
    %6 = FUNC<_lambda_8_17>
    *(%7 + 0) = %6
    *(%7 + 4) = %0
    *(%7 + 8) = %2
    *(%7 + 12) = %1
    %5 = %7
    %9 = *(%2 + 0)
    parm %2
    parm 120
    %10 = call %9
    %8 = *(%5 + 0)
    parm %5
    parm %10
    call %8
    %11 = *(%5 + 0)
    parm %5
    parm 13
    call %11
    parm 8
    %14 = call _Alloc
    %13 = FUNC<_lambda_12_23>
    *(%14 + 0) = %13
    *(%14 + 4) = %0
    %12 = %14
    parm 16
    %17 = call _Alloc
    %16 = FUNC<_lambda_13_29>
    *(%17 + 0) = %16
    *(%17 + 4) = %0
    *(%17 + 8) = %2
    *(%17 + 12) = %1
    %15 = %17
    %19 = *(%12 + 0)
    parm %12
    parm 120
    %20 = call %19
    %22 = *(%12 + 0)
    parm %12
    parm %1
    %23 = call %22
    %21 = *(%2 + 0)
    parm %2
    parm %23
    %24 = call %21
    %18 = *(%15 + 0)
    parm %15
    parm %20
    parm %24
    call %18
    %30 = *(%2 + 0)
    parm %2
    parm 2333
    %31 = call %30
    %29 = *(%12 + 0)
    parm %12
    parm %31
    %32 = call %29
    %28 = *(%2 + 0)
    parm %2
    parm %32
    %33 = call %28
    %27 = *(%12 + 0)
    parm %12
    parm %33
    %34 = call %27
    %26 = *(%2 + 0)
    parm %2
    parm %34
    %35 = call %26
    %25 = *(%15 + 0)
    parm %15
    parm 13
    parm %35
    call %25
    return
}

FUNC<main> {
    parm 8
    %2 = call _Alloc
    %1 = FUNC<_lambda_19_20>
    *(%2 + 0) = %1
    *(%2 + 4) = %0
    %0 = %2
    parm 8
    %5 = call _Alloc
    %4 = FUNC<_lambda_20_25>
    *(%5 + 0) = %4
    *(%5 + 4) = %0
    %3 = %5
    parm 12
    %8 = call _Alloc
    %7 = FUNC<_lambda_21_23>
    *(%8 + 0) = %7
    *(%8 + 4) = %0
    *(%8 + 8) = %0
    %6 = %8
    parm 8
    %11 = call _Alloc
    %10 = FUNC<_lambda_22_20>
    *(%11 + 0) = %10
    *(%11 + 4) = %0
    %9 = %11
    %14 = *(%6 + 0)
    parm %6
    parm 2
    %15 = call %14
    %13 = *(%0 + 0)
    parm %0
    parm 2
    parm %15
    %16 = call %13
    %12 = %16
    %17 = *(%0 + 0)
    parm %0
    parm 2
    parm 3
    %18 = call %17
    parm %18
    call _PrintInt
    %19 = *(%3 + 0)
    parm %3
    parm 4
    %20 = call %19
    parm %20
    call _PrintInt
    %21 = *(%6 + 0)
    parm %6
    parm 6
    %22 = call %21
    parm %22
    call _PrintInt
    %23 = *(%9 + 0)
    parm %9
    parm 8
    %24 = call %23
    parm %24
    call _PrintInt
    %25 = *(%3 + 0)
    parm %3
    parm 9
    %26 = call %25
    parm %26
    call _PrintInt
    parm %12
    call _PrintInt
    return
}

FUNC<_lambda_7_17> {
    %2 = (%1 + 1)
    return %2
}

FUNC<_lambda_8_17> {
    %2 = *(%0 + 8)
    %3 = *(%0 + 12)
    %5 = (%1 + %3)
    %4 = *(%2 + 0)
    parm %2
    parm %5
    %6 = call %4
    parm %6
    call _PrintInt
    return
}

FUNC<_lambda_12_23> {
    %2 = (%1 + 1)
    return %2
}

FUNC<_lambda_13_29> {
    %3 = *(%0 + 8)
    %4 = *(%0 + 12)
    %6 = (%1 + %4)
    %7 = (%6 + %2)
    %5 = *(%3 + 0)
    parm %3
    parm %7
    %8 = call %5
    parm %8
    call _PrintInt
    return
}

FUNC<_lambda_19_20> {
    %3 = (%1 + %2)
    return %3
}

FUNC<_lambda_20_25> {
    %2 = (%1 + 5)
    return %2
}

FUNC<_lambda_21_23> {
    %2 = *(%0 + 8)
    %3 = *(%2 + 0)
    parm %2
    parm 2
    parm 3
    %4 = call %3
    %5 = (%1 + %4)
    return %5
}

FUNC<_lambda_22_20> {
    %2 = (3 + %1)
    return %2
}

