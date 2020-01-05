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
    parm 404
    %4 = call _Alloc
    %2 = (%4 + 404)
    %4 = (%4 + 4)
    branch %3
    %2:
    %2 = (%2 - 4)
    *(%2 + 0) = 0
    %3:
    %1 = (%2 == %4)
    if (%1 == 0) branch %2
    *(%4 - 4) = 100
    *(%0 + 8) = %4
    *(%0 + 4) = 0
    parm %0
    parm 3
    %5 = *(%0 + 0)
    %5 = *(%5 + 12)
    call %5
    return
}

FUNC<_Stack.Push> {
    %2 = *(%0 + 8)
    %11 = *(%0 + 4)
    %5 = *(%2 - 4)
    %4 = (%11 >= 0)
    %6 = (%11 < %5)
    %4 = (%4 && %6)
    if (%4 == 0) branch %2
    %7 = (%11 * 4)
    %7 = (%7 + %2)
    *(%7 + 0) = %1
    branch %3
    %2:
    %8 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %8
    call _PrintString
    call _Halt
    %3:
    %10 = (%11 + 1)
    *(%0 + 4) = %10
    return
}

FUNC<_Stack.Pop> {
    %2 = *(%0 + 8)
    %13 = *(%0 + 4)
    %14 = (%13 - 1)
    %6 = *(%2 - 4)
    %5 = (%14 >= 0)
    %7 = (%14 < %6)
    %5 = (%5 && %7)
    if (%5 == 0) branch %2
    %8 = (%14 * 4)
    %8 = (%8 + %2)
    %9 = *(%8 + 0)
    branch %3
    %2:
    %10 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %10
    call _PrintString
    call _Halt
    %3:
    *(%0 + 4) = %14
    return %9
}

FUNC<_Stack.NumElems> {
    %1 = *(%0 + 4)
    return %1
}

FUNC<_Stack.main> {
    %1 = call _Stack._new
    parm %1
    %20 = *(%1 + 0)
    %2 = *(%20 + 8)
    call %2
    parm %1
    parm 3
    %28 = *(%20 + 12)
    call %28
    parm %1
    parm 7
    call %28
    parm %1
    parm 4
    call %28
    parm %1
    %32 = *(%20 + 20)
    %6 = call %32
    parm %6
    call _PrintInt
    %8 = " "
    parm %8
    call _PrintString
    parm %1
    %30 = *(%20 + 16)
    %9 = call %30
    parm %9
    call _PrintInt
    %11 = " "
    parm %11
    call _PrintString
    parm %1
    %12 = call %30
    parm %12
    call _PrintInt
    %14 = " "
    parm %14
    call _PrintString
    parm %1
    %15 = call %30
    parm %15
    call _PrintInt
    %17 = " "
    parm %17
    call _PrintString
    parm %1
    %18 = call %32
    parm %18
    call _PrintInt
    return
}

FUNC<main> {
    call _Stack.main
    return
}

