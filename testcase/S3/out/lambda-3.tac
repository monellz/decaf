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

FUNC<_Main.f1> {
    parm 2333
    call _PrintInt
    return
}

FUNC<_Main.f2> {
    %0 = "3222"
    parm %0
    call _PrintString
    return
}

FUNC<main> {
    parm 8
    %2 = call _Alloc
    %1 = FUNC<_lambda_10_19>
    *(%2 + 0) = %1
    *(%2 + 4) = %0
    %0 = %2
    %3 = *(%0 + 0)
    parm %0
    parm 100
    %4 = call %3
    parm %4
    call _PrintInt
    %6 = - 100
    %5 = *(%0 + 0)
    parm %0
    parm %6
    %7 = call %5
    parm %7
    call _PrintInt
    parm 8
    %10 = call _Alloc
    %9 = FUNC<_lambda_19_21>
    *(%10 + 0) = %9
    *(%10 + 4) = %0
    %8 = %10
    %11 = *(%8 + 0)
    parm %8
    parm 100
    call %11
    %13 = - 100
    %12 = *(%8 + 0)
    parm %8
    parm %13
    call %12
    %15 = (10 < 0)
    if (%15 == 0) branch %0
    %17 = "Decaf runtime error: Cannot create negative-sized array\n"
    parm %17
    call _PrintString
    call _Halt
    %0:
    %16 = (10 * 4)
    %16 = (%16 + 4)
    parm %16
    %18 = call _Alloc
    %16 = (%18 + %16)
    %18 = (%18 + 4)
    branch %1
    %2:
    %16 = (%16 - 4)
    *(%16 + 0) = 0
    %1:
    %15 = (%16 == %18)
    if (%15 == 0) branch %2
    *(%18 - 4) = 10
    %14 = %18
    %20 = *(%14 - 4)
    %19 = (0 >= 0)
    %21 = (0 < %20)
    %19 = (%19 && %21)
    if (%19 == 0) branch %3
    %22 = (0 * 4)
    %22 = (%22 + %14)
    *(%22 + 0) = 1
    branch %4
    %3:
    %23 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %23
    call _PrintString
    call _Halt
    %4:
    %25 = *(%14 - 4)
    %24 = (1 >= 0)
    %26 = (1 < %25)
    %24 = (%24 && %26)
    if (%24 == 0) branch %5
    %27 = (1 * 4)
    %27 = (%27 + %14)
    *(%27 + 0) = 3
    branch %6
    %5:
    %28 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %28
    call _PrintString
    call _Halt
    %6:
    %30 = *(%14 - 4)
    %29 = (2 >= 0)
    %31 = (2 < %30)
    %29 = (%29 && %31)
    if (%29 == 0) branch %7
    %32 = (2 * 4)
    %32 = (%32 + %14)
    *(%32 + 0) = 5
    branch %8
    %7:
    %33 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %33
    call _PrintString
    call _Halt
    %8:
    %35 = *(%14 - 4)
    %34 = (3 >= 0)
    %36 = (3 < %35)
    %34 = (%34 && %36)
    if (%34 == 0) branch %9
    %37 = (3 * 4)
    %37 = (%37 + %14)
    *(%37 + 0) = 7
    branch %10
    %9:
    %38 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %38
    call _PrintString
    call _Halt
    %10:
    %40 = *(%14 - 4)
    %39 = (4 >= 0)
    %41 = (4 < %40)
    %39 = (%39 && %41)
    if (%39 == 0) branch %11
    %42 = (4 * 4)
    %42 = (%42 + %14)
    *(%42 + 0) = 9
    branch %12
    %11:
    %43 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %43
    call _PrintString
    call _Halt
    %12:
    %45 = *(%14 - 4)
    %44 = (5 >= 0)
    %46 = (5 < %45)
    %44 = (%44 && %46)
    if (%44 == 0) branch %13
    %47 = (5 * 4)
    %47 = (%47 + %14)
    *(%47 + 0) = 0
    branch %14
    %13:
    %48 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %48
    call _PrintString
    call _Halt
    %14:
    parm 8
    %51 = call _Alloc
    %50 = FUNC<_lambda_32_22>
    *(%51 + 0) = %50
    *(%51 + 4) = %0
    %49 = %51
    %52 = *(%49 + 0)
    parm %49
    parm 10
    parm %14
    call %52
    parm 8
    %55 = call _Alloc
    %54 = FUNC<_lambda_42_17>
    *(%55 + 0) = %54
    *(%55 + 4) = %0
    %53 = %55
    %56 = *(%53 + 0)
    parm %53
    parm 1
    %57 = call %56
    %58 = *(%57 + 0)
    parm %57
    call %58
    %59 = *(%53 + 0)
    parm %53
    parm 0
    %60 = call %59
    %61 = *(%60 + 0)
    parm %60
    call %61
    return
}

FUNC<_lambda_10_19> {
    %2 = (%1 >= 0)
    if (%2 == 0) branch %0
    return %1
    branch %1
    %0:
    %3 = - %1
    return %3
    %1:
}

FUNC<_lambda_19_21> {
    %2 = (%1 < 0)
    if (%2 == 0) branch %0
    return
    %0:
    parm %1
    call _PrintInt
    return
}

FUNC<_lambda_32_22> {
    %3 = 0
    branch %0
    %1:
    %5 = *(%2 - 4)
    %4 = (%3 >= 0)
    %6 = (%3 < %5)
    %4 = (%4 && %6)
    if (%4 == 0) branch %4
    %7 = (%3 * 4)
    %7 = (%7 + %2)
    %8 = *(%7 + 0)
    branch %5
    %4:
    %9 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %9
    call _PrintString
    call _Halt
    %5:
    %10 = (%8 == 0)
    if (%10 == 0) branch %3
    return
    %3:
    %12 = *(%2 - 4)
    %11 = (%3 >= 0)
    %13 = (%3 < %12)
    %11 = (%11 && %13)
    if (%11 == 0) branch %6
    %14 = (%3 * 4)
    %14 = (%14 + %2)
    %15 = *(%14 + 0)
    branch %7
    %6:
    %16 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %16
    call _PrintString
    call _Halt
    %7:
    parm %15
    call _PrintInt
    %17 = (%3 + 1)
    %3 = %17
    %0:
    %18 = (%3 < %1)
    if (%18 != 0) branch %1
    %2:
    return
}

FUNC<_f1_44_24> {
    call _Main.f1
    return
}

FUNC<_f2_46_24> {
    call _Main.f2
    return
}

FUNC<_lambda_42_17> {
    if (%1 == 0) branch %0
    parm 4
    %3 = call _Alloc
    %2 = FUNC<_f1_44_24>
    *(%3 + 0) = %2
    return %3
    branch %1
    %0:
    parm 4
    %5 = call _Alloc
    %4 = FUNC<_f2_46_24>
    *(%5 + 0) = %4
    return %5
    %1:
}

