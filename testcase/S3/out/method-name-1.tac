VTBL<_Main> {
    0
    "Main"
    FUNC<_Main.f1>
    FUNC<_Main.f2>
    FUNC<_Main.f3>
    FUNC<_Main.f4>
    FUNC<_Main.f5>
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
    %0 = %1
    parm 8
    %5 = call _Alloc
    %2 = FUNC<_f5_5_11>
    *(%5 + 0) = %2
    *(%5 + 4) = %0
    %6 = *(%5 + 0)
    parm %5
    call %6
    parm 8
    %11 = call _Alloc
    %7 = FUNC<_f4_6_11>
    *(%11 + 0) = %7
    *(%11 + 4) = %0
    %12 = *(%11 + 0)
    parm %11
    %13 = call %12
    %14 = *(%13 + 0)
    parm %13
    call %14
    parm 8
    %19 = call _Alloc
    %15 = FUNC<_f3_7_11>
    *(%19 + 0) = %15
    *(%19 + 4) = %0
    %20 = *(%19 + 0)
    parm %19
    %21 = call %20
    %22 = *(%21 + 0)
    parm %21
    %23 = call %22
    %24 = *(%23 + 0)
    parm %23
    call %24
    parm 8
    %29 = call _Alloc
    %25 = FUNC<_f2_8_11>
    *(%29 + 0) = %25
    *(%29 + 4) = %0
    %30 = *(%29 + 0)
    parm %29
    %31 = call %30
    %32 = *(%31 + 0)
    parm %31
    %33 = call %32
    %34 = *(%33 + 0)
    parm %33
    %35 = call %34
    %36 = *(%35 + 0)
    parm %35
    call %36
    parm 8
    %41 = call _Alloc
    %37 = FUNC<_f1_9_11>
    *(%41 + 0) = %37
    *(%41 + 4) = %0
    %42 = *(%41 + 0)
    parm %41
    %43 = call %42
    %44 = *(%43 + 0)
    parm %43
    %45 = call %44
    %46 = *(%45 + 0)
    parm %45
    %47 = call %46
    %48 = *(%47 + 0)
    parm %47
    %49 = call %48
    %50 = *(%49 + 0)
    parm %49
    call %50
    return
}

FUNC<_Main.f1> {
    %1 = *(%0 + 4)
    %2 = (%1 + 1)
    *(%0 + 4) = %2
    parm 8
    %7 = call _Alloc
    %3 = FUNC<_f2_12_43>
    *(%7 + 0) = %3
    *(%7 + 4) = %0
    return %7
}

FUNC<_Main.f2> {
    %1 = *(%0 + 4)
    %2 = (%1 + 1)
    *(%0 + 4) = %2
    parm 8
    %7 = call _Alloc
    %3 = FUNC<_f3_13_41>
    *(%7 + 0) = %3
    *(%7 + 4) = %0
    return %7
}

FUNC<_Main.f3> {
    %1 = *(%0 + 4)
    %2 = (%1 + 1)
    *(%0 + 4) = %2
    parm 8
    %7 = call _Alloc
    %3 = FUNC<_f4_14_39>
    *(%7 + 0) = %3
    *(%7 + 4) = %0
    return %7
}

FUNC<_Main.f4> {
    %1 = *(%0 + 4)
    %2 = (%1 + 1)
    *(%0 + 4) = %2
    parm 8
    %6 = call _Alloc
    %3 = FUNC<_f5_15_37>
    *(%6 + 0) = %3
    *(%6 + 4) = %0
    return %6
}

FUNC<_Main.f5> {
    %1 = *(%0 + 4)
    %2 = (%1 + 1)
    *(%0 + 4) = %2
    %3 = *(%0 + 4)
    parm %3
    call _PrintInt
    %4 = "\n"
    parm %4
    call _PrintString
    return
}

FUNC<_f5_5_11> {
    %3 = *(%0 + 4)
    parm %3
    %4 = *(%3 + 0)
    %4 = *(%4 + 24)
    call %4
    return
}

FUNC<_f4_6_11> {
    %9 = *(%0 + 4)
    parm %9
    %10 = *(%9 + 0)
    %10 = *(%10 + 20)
    %8 = call %10
    return %8
}

FUNC<_f3_7_11> {
    %17 = *(%0 + 4)
    parm %17
    %18 = *(%17 + 0)
    %18 = *(%18 + 16)
    %16 = call %18
    return %16
}

FUNC<_f2_8_11> {
    %27 = *(%0 + 4)
    parm %27
    %28 = *(%27 + 0)
    %28 = *(%28 + 12)
    %26 = call %28
    return %26
}

FUNC<_f1_9_11> {
    %39 = *(%0 + 4)
    parm %39
    %40 = *(%39 + 0)
    %40 = *(%40 + 8)
    %38 = call %40
    return %38
}

FUNC<_f2_12_43> {
    %5 = *(%0 + 4)
    parm %5
    %6 = *(%5 + 0)
    %6 = *(%6 + 12)
    %4 = call %6
    return %4
}

FUNC<_f3_13_41> {
    %5 = *(%0 + 4)
    parm %5
    %6 = *(%5 + 0)
    %6 = *(%6 + 16)
    %4 = call %6
    return %4
}

FUNC<_f4_14_39> {
    %5 = *(%0 + 4)
    parm %5
    %6 = *(%5 + 0)
    %6 = *(%6 + 20)
    %4 = call %6
    return %4
}

FUNC<_f5_15_37> {
    %4 = *(%0 + 4)
    parm %4
    %5 = *(%4 + 0)
    %5 = *(%5 + 24)
    call %5
    return
}

