VTBL<_Main> {
    0
    "Main"
    FUNC<_Main.foo>
}

FUNC<_Main._new> {
    parm 12
    %0 = call _Alloc
    %1 = VTBL<_Main>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    *(%0 + 8) = 0
    return %0
}

FUNC<_Main.foo> {
    %2 = "PA3"
    %1 = %2
    parm 16
    %5 = call _Alloc
    %4 = FUNC<_lambda_7_17>
    *(%5 + 0) = %4
    *(%5 + 4) = %0
    *(%5 + 8) = %1
    *(%5 + 12) = %1
    %3 = %5
    parm %1
    call _PrintString
    %6 = "\n"
    parm %6
    call _PrintString
    %7 = *(%3 + 0)
    parm %3
    call %7
    return
}

FUNC<main> {
    %1 = (10 < 0)
    if (%1 == 0) branch %0
    %3 = "Decaf runtime error: Cannot create negative-sized array\n"
    parm %3
    call _PrintString
    call _Halt
    %0:
    %2 = (10 * 4)
    %2 = (%2 + 4)
    parm %2
    %4 = call _Alloc
    %2 = (%4 + %2)
    %4 = (%4 + 4)
    branch %1
    %2:
    %2 = (%2 - 4)
    *(%2 + 0) = 0
    %1:
    %1 = (%2 == %4)
    if (%1 == 0) branch %2
    *(%4 - 4) = 10
    %0 = %4
    %6 = call _Main._new
    %5 = %6
    *(%5 + 4) = 3
    %8 = *(%0 - 4)
    %7 = (0 >= 0)
    %9 = (0 < %8)
    %7 = (%7 && %9)
    if (%7 == 0) branch %3
    %10 = (0 * 4)
    %10 = (%10 + %0)
    %11 = *(%10 + 0)
    branch %4
    %3:
    %12 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %12
    call _PrintString
    call _Halt
    %4:
    parm %11
    call _PrintInt
    %13 = "\n"
    parm %13
    call _PrintString
    %14 = *(%5 + 4)
    parm %14
    call _PrintInt
    %15 = "\n"
    parm %15
    call _PrintString
    parm 20
    %18 = call _Alloc
    %17 = FUNC<_lambda_37_20>
    *(%18 + 0) = %17
    *(%18 + 4) = %0
    *(%18 + 8) = %0
    *(%18 + 12) = %0
    *(%18 + 16) = %5
    %16 = %18
    %19 = *(%16 + 0)
    parm %16
    call %19
    %21 = *(%0 - 4)
    %20 = (0 >= 0)
    %22 = (0 < %21)
    %20 = (%20 && %22)
    if (%20 == 0) branch %5
    %23 = (0 * 4)
    %23 = (%23 + %0)
    %24 = *(%23 + 0)
    branch %6
    %5:
    %25 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %25
    call _PrintString
    call _Halt
    %6:
    parm %24
    call _PrintInt
    %26 = "\n"
    parm %26
    call _PrintString
    %27 = *(%5 + 4)
    parm %27
    call _PrintInt
    %28 = "\n"
    parm %28
    call _PrintString
    parm 8
    %32 = call _Alloc
    %29 = FUNC<_foo_50_11>
    *(%32 + 0) = %29
    *(%32 + 4) = %5
    %33 = *(%32 + 0)
    parm %32
    call %33
    %34 = *(%5 + 4)
    parm %34
    call _PrintInt
    %35 = "\n"
    parm %35
    call _PrintString
    return
}

FUNC<_lambda_9_21> {
    %3 = *(%0 + 4)
    *(%3 + 4) = 4
    %4 = *(%0 + 4)
    *(%4 + 8) = 0
    %6 = "fzyxqzgbyq"
    %5 = %6
    %5 = %2
    parm %5
    parm %1
    %7 = call _StringEqual
    %7 = ! %7
    if (%7 == 0) branch %0
    %5 = %1
    %1 = %2
    %2 = %5
    %0:
    return %5
}

FUNC<_lambda_7_17> {
    %1 = *(%0 + 8)
    %2 = *(%0 + 12)
    %4 = "PA3"
    %3 = %4
    parm 8
    %7 = call _Alloc
    %6 = FUNC<_lambda_9_21>
    *(%7 + 0) = %6
    %8 = *(%0 + 4)
    *(%7 + 4) = %8
    %5 = %7
    %9 = *(%5 + 0)
    parm %5
    parm %2
    parm %3
    %10 = call %9
    parm %10
    call _PrintString
    %11 = "\n"
    parm %11
    call _PrintString
    %12 = *(%0 + 4)
    %13 = *(%12 + 4)
    parm %13
    call _PrintInt
    %14 = "\n"
    parm %14
    call _PrintString
    %3 = %2
    %15 = *(%0 + 4)
    *(%15 + 4) = 2
    return
    return
}

FUNC<_lambda_40_24> {
    %2 = *(%0 + 8)
    %3 = *(%0 + 12)
    %1 = %2
    *(%3 + 4) = 1
    return
}

FUNC<_lambda_37_20> {
    %1 = *(%0 + 8)
    %2 = *(%0 + 12)
    %3 = *(%0 + 16)
    %5 = *(%2 - 4)
    %4 = (0 >= 0)
    %6 = (0 < %5)
    %4 = (%4 && %6)
    if (%4 == 0) branch %0
    %7 = (0 * 4)
    %7 = (%7 + %2)
    %8 = *(%7 + 0)
    branch %1
    %0:
    %9 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %9
    call _PrintString
    call _Halt
    %1:
    %10 = (%8 + 1)
    %12 = *(%2 - 4)
    %11 = (0 >= 0)
    %13 = (0 < %12)
    %11 = (%11 && %13)
    if (%11 == 0) branch %2
    %14 = (0 * 4)
    %14 = (%14 + %2)
    *(%14 + 0) = %10
    branch %3
    %2:
    %15 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %15
    call _PrintString
    call _Halt
    %3:
    %16 = 0
    parm 16
    %19 = call _Alloc
    %18 = FUNC<_lambda_40_24>
    *(%19 + 0) = %18
    %20 = *(%0 + 4)
    *(%19 + 4) = %20
    *(%19 + 8) = %16
    *(%19 + 12) = %3
    %17 = %19
    %21 = - 1
    %16 = %21
    %22 = *(%17 + 0)
    parm %17
    parm 1
    call %22
    return
}

FUNC<_foo_50_11> {
    %30 = *(%0 + 4)
    parm %30
    %31 = *(%30 + 0)
    %31 = *(%31 + 8)
    call %31
    return
}

