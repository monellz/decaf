VTBL<_Stack> {
    0
    "Stack"
    FUNC<_Stack.Init>
    FUNC<_Stack.Push>
    FUNC<_Stack.Pop>
    FUNC<_Stack.NumElems>
}

VTBL<_Main> {
    0
    "Main"
}

FUNC<_Stack._new> {
    parm 12
    %0 = call _Alloc
    %1 = VTBL<_Stack>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    *(%0 + 8) = 0
    return %0
}

FUNC<_Main._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Main>
    *(%0 + 0) = %1
    return %0
}

FUNC<_Stack.Init> {
    %1 = (100 < 0)
    if (%1 == 0) branch %0
    %3 = "Decaf runtime error: Cannot create negative-sized array\n"
    parm %3
    call _PrintString
    call _Halt
    %0:
    %2 = (100 * 4)
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
    *(%4 - 4) = 100
    *(%0 + 8) = %4
    *(%0 + 4) = 0
    parm 8
    %8 = call _Alloc
    %5 = FUNC<_Push_8_9>
    *(%8 + 0) = %5
    *(%8 + 4) = %0
    %9 = *(%8 + 0)
    parm %8
    parm 3
    call %9
    return
}

FUNC<_Stack.Push> {
    %2 = *(%0 + 8)
    %3 = *(%0 + 4)
    %5 = *(%2 - 4)
    %4 = (%3 >= 0)
    %6 = (%3 < %5)
    %4 = (%4 && %6)
    if (%4 == 0) branch %0
    %7 = (%3 * 4)
    %7 = (%7 + %2)
    *(%7 + 0) = %1
    branch %1
    %0:
    %8 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %8
    call _PrintString
    call _Halt
    %1:
    %9 = *(%0 + 4)
    %10 = (%9 + 1)
    *(%0 + 4) = %10
    return
}

FUNC<_Stack.Pop> {
    %2 = *(%0 + 8)
    %3 = *(%0 + 4)
    %4 = (%3 - 1)
    %6 = *(%2 - 4)
    %5 = (%4 >= 0)
    %7 = (%4 < %6)
    %5 = (%5 && %7)
    if (%5 == 0) branch %0
    %8 = (%4 * 4)
    %8 = (%8 + %2)
    %9 = *(%8 + 0)
    branch %1
    %0:
    %10 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %10
    call _PrintString
    call _Halt
    %1:
    %1 = %9
    %11 = *(%0 + 4)
    %12 = (%11 - 1)
    *(%0 + 4) = %12
    return %1
}

FUNC<_Stack.NumElems> {
    %1 = *(%0 + 4)
    return %1
}

FUNC<_Stack.main> {
    %1 = call _Stack._new
    %0 = %1
    parm 8
    %5 = call _Alloc
    %2 = FUNC<_Init_28_11>
    *(%5 + 0) = %2
    *(%5 + 4) = %0
    %6 = *(%5 + 0)
    parm %5
    call %6
    parm 8
    %10 = call _Alloc
    %7 = FUNC<_Push_29_11>
    *(%10 + 0) = %7
    *(%10 + 4) = %0
    %11 = *(%10 + 0)
    parm %10
    parm 3
    call %11
    parm 8
    %15 = call _Alloc
    %12 = FUNC<_Push_30_11>
    *(%15 + 0) = %12
    *(%15 + 4) = %0
    %16 = *(%15 + 0)
    parm %15
    parm 7
    call %16
    parm 8
    %20 = call _Alloc
    %17 = FUNC<_Push_31_11>
    *(%20 + 0) = %17
    *(%20 + 4) = %0
    %21 = *(%20 + 0)
    parm %20
    parm 4
    call %21
    parm 8
    %26 = call _Alloc
    %22 = FUNC<_NumElems_32_17>
    *(%26 + 0) = %22
    *(%26 + 4) = %0
    %27 = *(%26 + 0)
    parm %26
    %28 = call %27
    parm %28
    call _PrintInt
    %29 = " "
    parm %29
    call _PrintString
    parm 8
    %34 = call _Alloc
    %30 = FUNC<_Pop_32_36>
    *(%34 + 0) = %30
    *(%34 + 4) = %0
    %35 = *(%34 + 0)
    parm %34
    %36 = call %35
    parm %36
    call _PrintInt
    %37 = " "
    parm %37
    call _PrintString
    parm 8
    %42 = call _Alloc
    %38 = FUNC<_Pop_32_50>
    *(%42 + 0) = %38
    *(%42 + 4) = %0
    %43 = *(%42 + 0)
    parm %42
    %44 = call %43
    parm %44
    call _PrintInt
    %45 = " "
    parm %45
    call _PrintString
    parm 8
    %50 = call _Alloc
    %46 = FUNC<_Pop_32_64>
    *(%50 + 0) = %46
    *(%50 + 4) = %0
    %51 = *(%50 + 0)
    parm %50
    %52 = call %51
    parm %52
    call _PrintInt
    %53 = " "
    parm %53
    call _PrintString
    parm 8
    %58 = call _Alloc
    %54 = FUNC<_NumElems_32_78>
    *(%58 + 0) = %54
    *(%58 + 4) = %0
    %59 = *(%58 + 0)
    parm %58
    %60 = call %59
    parm %60
    call _PrintInt
    return
}

FUNC<main> {
    parm 4
    %1 = call _Alloc
    %0 = FUNC<_main_38_15>
    *(%1 + 0) = %0
    %2 = *(%1 + 0)
    parm %1
    call %2
    return
}

FUNC<_Push_8_9> {
    %6 = *(%0 + 4)
    parm %6
    parm %1
    %7 = *(%6 + 0)
    %7 = *(%7 + 12)
    call %7
    return
}

FUNC<_Init_28_11> {
    %3 = *(%0 + 4)
    parm %3
    %4 = *(%3 + 0)
    %4 = *(%4 + 8)
    call %4
    return
}

FUNC<_Push_29_11> {
    %8 = *(%0 + 4)
    parm %8
    parm %1
    %9 = *(%8 + 0)
    %9 = *(%9 + 12)
    call %9
    return
}

FUNC<_Push_30_11> {
    %13 = *(%0 + 4)
    parm %13
    parm %1
    %14 = *(%13 + 0)
    %14 = *(%14 + 12)
    call %14
    return
}

FUNC<_Push_31_11> {
    %18 = *(%0 + 4)
    parm %18
    parm %1
    %19 = *(%18 + 0)
    %19 = *(%19 + 12)
    call %19
    return
}

FUNC<_NumElems_32_17> {
    %24 = *(%0 + 4)
    parm %24
    %25 = *(%24 + 0)
    %25 = *(%25 + 20)
    %23 = call %25
    return %23
}

FUNC<_Pop_32_36> {
    %32 = *(%0 + 4)
    parm %32
    %33 = *(%32 + 0)
    %33 = *(%33 + 16)
    %31 = call %33
    return %31
}

FUNC<_Pop_32_50> {
    %40 = *(%0 + 4)
    parm %40
    %41 = *(%40 + 0)
    %41 = *(%41 + 16)
    %39 = call %41
    return %39
}

FUNC<_Pop_32_64> {
    %48 = *(%0 + 4)
    parm %48
    %49 = *(%48 + 0)
    %49 = *(%49 + 16)
    %47 = call %49
    return %47
}

FUNC<_NumElems_32_78> {
    %56 = *(%0 + 4)
    parm %56
    %57 = *(%56 + 0)
    %57 = *(%57 + 20)
    %55 = call %57
    return %55
}

FUNC<_main_38_15> {
    call _Stack.main
    return
}

