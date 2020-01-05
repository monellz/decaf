VTBL<_Main> {
    0
    "Main"
}

VTBL<_Rng> {
    0
    "Rng"
    FUNC<_Rng.next>
}

VTBL<_QuickSort> {
    0
    "QuickSort"
}

VTBL<_MergeSort> {
    0
    "MergeSort"
}

FUNC<_Main._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Main>
    *(%0 + 0) = %1
    return %0
}

FUNC<_Rng._new> {
    parm 8
    %0 = call _Alloc
    %1 = VTBL<_Rng>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    return %0
}

FUNC<_QuickSort._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_QuickSort>
    *(%0 + 0) = %1
    return %0
}

FUNC<_MergeSort._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_MergeSort>
    *(%0 + 0) = %1
    return %0
}

FUNC<main> {
    parm 19260817
    %1 = call _Rng.make
    parm 2004
    %6 = call _Alloc
    %4 = (%6 + 2004)
    %6 = (%6 + 4)
    branch %3
    %2:
    %4 = (%4 - 4)
    *(%4 + 0) = 0
    %3:
    %3 = (%4 == %6)
    if (%3 == 0) branch %2
    *(%6 - 4) = 500
    parm 2004
    %11 = call _Alloc
    %9 = (%11 + 2004)
    %11 = (%11 + 4)
    branch %7
    %6:
    %9 = (%9 - 4)
    *(%9 + 0) = 0
    %7:
    %8 = (%9 == %11)
    if (%8 == 0) branch %6
    *(%11 - 4) = 500
    %12 = 0
    branch %19
    %9:
    parm %1
    %14 = *(%1 + 0)
    %14 = *(%14 + 8)
    %13 = call %14
    %15 = (%13 % 500)
    %65 = (%12 >= 0)
    %77 = (%65 && %73)
    if (%77 == 0) branch %11
    %66 = (%12 * 4)
    %75 = (%66 + %6)
    *(%75 + 0) = %15
    branch %12
    %11:
    %20 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %20
    call _PrintString
    call _Halt
    %12:
    if (%77 == 0) branch %14
    %25 = *(%75 + 0)
    branch %15
    %14:
    %26 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %26
    call _PrintString
    call _Halt
    %15:
    %28 = *(%11 - 4)
    %29 = (%12 < %28)
    %27 = (%65 && %29)
    if (%27 == 0) branch %17
    %30 = (%66 + %11)
    *(%30 + 0) = %25
    branch %18
    %17:
    %31 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %31
    call _PrintString
    call _Halt
    %18:
    %32 = (%12 + 1)
    %12 = %32
    %19:
    %69 = *(%6 - 4)
    %73 = (%12 < %69)
    if (%73 != 0) branch %9
    %36 = (%69 - 1)
    parm %6
    parm 0
    parm %36
    call _QuickSort.sort
    %37 = 0
    branch %25
    %21:
    %38 = (%37 >= 0)
    %38 = (%38 && %78)
    if (%38 == 0) branch %23
    %41 = (%37 * 4)
    %41 = (%41 + %6)
    %42 = *(%41 + 0)
    branch %24
    %23:
    %43 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %43
    call _PrintString
    call _Halt
    %24:
    parm %42
    call _PrintInt
    %44 = " "
    parm %44
    call _PrintString
    %45 = (%37 + 1)
    %37 = %45
    %25:
    %78 = (%37 < %69)
    if (%78 != 0) branch %21
    %48 = "\n"
    parm %48
    call _PrintString
    parm %11
    call _MergeSort.sort
    %49 = 0
    branch %31
    %27:
    %50 = (%49 >= 0)
    %50 = (%50 && %76)
    if (%50 == 0) branch %29
    %53 = (%49 * 4)
    %53 = (%53 + %11)
    %54 = *(%53 + 0)
    branch %30
    %29:
    %55 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %55
    call _PrintString
    call _Halt
    %30:
    parm %54
    call _PrintInt
    %56 = " "
    parm %56
    call _PrintString
    %57 = (%49 + 1)
    %49 = %57
    %31:
    %72 = *(%11 - 4)
    %76 = (%49 < %72)
    if (%76 != 0) branch %27
    %60 = "\n"
    parm %60
    call _PrintString
    return
}

FUNC<_Rng.make> {
    %2 = call _Rng._new
    *(%2 + 4) = %0
    return %2
}

FUNC<_Rng.next> {
    %1 = *(%0 + 4)
    %2 = (%1 % 10000)
    %3 = (15625 * %2)
    %4 = (%3 + 22221)
    %5 = (%4 % 65536)
    *(%0 + 4) = %5
    %6 = *(%0 + 4)
    return %6
}

FUNC<_QuickSort.sort> {
    %3 = %1
    %4 = %2
    %6 = (%2 - %1)
    %7 = (%6 / 2)
    %8 = (%1 + %7)
    %60 = *(%0 - 4)
    %9 = (%8 >= 0)
    %11 = (%8 < %60)
    %9 = (%9 && %11)
    if (%9 == 0) branch %2
    %12 = (%8 * 4)
    %12 = (%12 + %0)
    %13 = *(%12 + 0)
    branch %3
    %2:
    %14 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %14
    call _PrintString
    call _Halt
    %3:
    branch %30
    %4:
    branch %6
    %5:
    %15 = (%3 + 1)
    %3 = %15
    %6:
    %61 = %60
    %63 = (%3 >= 0)
    %18 = (%3 < %60)
    %16 = (%63 && %18)
    if (%16 == 0) branch %8
    %64 = (%3 * 4)
    %74 = (%64 + %0)
    %81 = *(%74 + 0)
    branch %9
    %8:
    %21 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %21
    call _PrintString
    call _Halt
    %9:
    %22 = (%81 < %13)
    if (%22 != 0) branch %5
    branch %12
    %11:
    %23 = (%4 - 1)
    %4 = %23
    %12:
    %60 = %61
    %66 = (%4 >= 0)
    %75 = (%4 < %61)
    %82 = (%66 && %75)
    if (%82 == 0) branch %14
    %67 = (%4 * 4)
    %76 = (%67 + %0)
    %83 = *(%76 + 0)
    branch %15
    %14:
    %29 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %29
    call _PrintString
    call _Halt
    %15:
    %30 = (%83 > %13)
    if (%30 != 0) branch %11
    %31 = (%3 <= %4)
    if (%31 == 0) branch %30
    %77 = (%3 < %61)
    %84 = (%63 && %77)
    if (%84 == 0) branch %19
    branch %20
    %19:
    %38 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %38
    call _PrintString
    call _Halt
    %20:
    if (%82 == 0) branch %22
    branch %23
    %22:
    %44 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %44
    call _PrintString
    call _Halt
    %23:
    if (%84 == 0) branch %25
    *(%74 + 0) = %83
    branch %26
    %25:
    %49 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %49
    call _PrintString
    call _Halt
    %26:
    %60 = %61
    if (%82 == 0) branch %28
    *(%76 + 0) = %81
    branch %29
    %28:
    %54 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %54
    call _PrintString
    call _Halt
    %29:
    %55 = (%3 + 1)
    %3 = %55
    %56 = (%4 - 1)
    %4 = %56
    %30:
    %57 = (%3 <= %4)
    if (%57 != 0) branch %4
    %58 = (%1 < %4)
    if (%58 == 0) branch %33
    parm %0
    parm %1
    parm %4
    call _QuickSort.sort
    %33:
    %59 = (%3 < %2)
    if (%59 == 0) branch %35
    parm %0
    parm %3
    parm %2
    call _QuickSort.sort
    %35:
    return
}

FUNC<_MergeSort.sort> {
    %7 = *(%0 - 4)
    %3 = (%7 < 0)
    if (%3 == 0) branch %2
    %5 = "Decaf runtime error: Cannot create negative-sized array\n"
    parm %5
    call _PrintString
    call _Halt
    %2:
    %4 = (%7 * 4)
    %4 = (%4 + 4)
    parm %4
    %6 = call _Alloc
    %4 = (%6 + %4)
    %6 = (%6 + 4)
    branch %4
    %3:
    %4 = (%4 - 4)
    *(%4 + 0) = 0
    %4:
    %3 = (%4 == %6)
    if (%3 == 0) branch %3
    *(%6 - 4) = %7
    parm %0
    parm 0
    parm %7
    parm %6
    call _MergeSort.sort_impl
    return
}

FUNC<_MergeSort.sort_impl> {
    %4 = (%1 + 1)
    %5 = (%4 < %2)
    if (%5 == 0) branch %43
    %7 = (%1 + %2)
    %8 = (%7 / 2)
    parm %0
    parm %1
    parm %8
    parm %3
    call _MergeSort.sort_impl
    parm %0
    parm %8
    parm %2
    parm %3
    call _MergeSort.sort_impl
    %9 = %1
    %10 = %8
    %11 = 0
    branch %24
    %2:
    %81 = *(%0 - 4)
    %83 = (%10 >= 0)
    %88 = (%10 < %81)
    %92 = (%83 && %88)
    if (%92 == 0) branch %4
    %84 = (%10 * 4)
    %89 = (%84 + %0)
    %93 = *(%89 + 0)
    branch %5
    %4:
    %17 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %17
    call _PrintString
    call _Halt
    %5:
    %86 = (%9 >= 0)
    %90 = (%9 < %81)
    %94 = (%86 && %90)
    if (%94 == 0) branch %7
    %87 = (%9 * 4)
    %91 = (%87 + %0)
    %95 = *(%91 + 0)
    branch %8
    %7:
    %23 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %23
    call _PrintString
    call _Halt
    %8:
    %24 = (%93 < %95)
    if (%24 == 0) branch %16
    if (%92 == 0) branch %11
    branch %12
    %11:
    %30 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %30
    call _PrintString
    call _Halt
    %12:
    %32 = *(%3 - 4)
    %31 = (%11 >= 0)
    %33 = (%11 < %32)
    %31 = (%31 && %33)
    if (%31 == 0) branch %14
    %34 = (%11 * 4)
    %34 = (%34 + %3)
    *(%34 + 0) = %93
    branch %15
    %14:
    %35 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %35
    call _PrintString
    call _Halt
    %15:
    %36 = (%10 + 1)
    %10 = %36
    branch %23
    %16:
    if (%94 == 0) branch %18
    branch %19
    %18:
    %42 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %42
    call _PrintString
    call _Halt
    %19:
    %44 = *(%3 - 4)
    %43 = (%11 >= 0)
    %45 = (%11 < %44)
    %43 = (%43 && %45)
    if (%43 == 0) branch %21
    %46 = (%11 * 4)
    %46 = (%46 + %3)
    *(%46 + 0) = %95
    branch %22
    %21:
    %47 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %47
    call _PrintString
    call _Halt
    %22:
    %48 = (%9 + 1)
    %9 = %48
    %23:
    %49 = (%11 + 1)
    %11 = %49
    %24:
    %50 = (%9 < %8)
    %51 = (%10 < %2)
    %52 = (%50 && %51)
    if (%52 != 0) branch %2
    branch %33
    %26:
    %54 = *(%0 - 4)
    %53 = (%9 >= 0)
    %55 = (%9 < %54)
    %53 = (%53 && %55)
    if (%53 == 0) branch %28
    %56 = (%9 * 4)
    %56 = (%56 + %0)
    %57 = *(%56 + 0)
    branch %29
    %28:
    %58 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %58
    call _PrintString
    call _Halt
    %29:
    %60 = *(%3 - 4)
    %59 = (%11 >= 0)
    %61 = (%11 < %60)
    %59 = (%59 && %61)
    if (%59 == 0) branch %31
    %62 = (%11 * 4)
    %62 = (%62 + %3)
    *(%62 + 0) = %57
    branch %32
    %31:
    %63 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %63
    call _PrintString
    call _Halt
    %32:
    %64 = (%11 + 1)
    %11 = %64
    %65 = (%9 + 1)
    %9 = %65
    %33:
    %66 = (%9 < %8)
    if (%66 != 0) branch %26
    %9 = 0
    branch %42
    %35:
    %68 = *(%3 - 4)
    %67 = (%9 >= 0)
    %69 = (%9 < %68)
    %67 = (%67 && %69)
    if (%67 == 0) branch %37
    %70 = (%9 * 4)
    %70 = (%70 + %3)
    %71 = *(%70 + 0)
    branch %38
    %37:
    %72 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %72
    call _PrintString
    call _Halt
    %38:
    %73 = (%9 + %1)
    %75 = *(%0 - 4)
    %74 = (%73 >= 0)
    %76 = (%73 < %75)
    %74 = (%74 && %76)
    if (%74 == 0) branch %40
    %77 = (%73 * 4)
    %77 = (%77 + %0)
    *(%77 + 0) = %71
    branch %41
    %40:
    %78 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %78
    call _PrintString
    call _Halt
    %41:
    %79 = (%9 + 1)
    %9 = %79
    %42:
    %80 = (%9 < %11)
    if (%80 != 0) branch %35
    %43:
    return
}

