VTBL<_A> {
    0
    "A"
    FUNC<_A.f>
}

VTBL<_Main> {
    0
    "Main"
    FUNC<_Main.f>
    FUNC<_Main.trueMain>
}

FUNC<_A._new> {
    parm 8
    %0 = call _Alloc
    %1 = VTBL<_A>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    return %0
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

FUNC<_A.f> {
    %2 = (%1 - 1)
    return %2
}

FUNC<_A.sf> {
    %1 = (%0 + 1)
    return %1
}

FUNC<_Main.f> {
    %2 = *(%0 + 4)
    %3 = (%1 + %2)
    return %3
}

FUNC<_Main.sf> {
    %1 = (%0 + 2)
    return %1
}

FUNC<main> {
    parm 4
    %3 = call _Alloc
    %1 = FUNC<_sf_15_23>
    *(%3 + 0) = %1
    %0 = %3
    %4 = *(%0 + 0)
    parm %0
    parm 3
    %5 = call %4
    parm %5
    call _PrintInt
    %7 = call _Main._new
    parm 8
    %10 = call _Alloc
    %6 = FUNC<_trueMain_17_20>
    *(%10 + 0) = %6
    *(%10 + 4) = %7
    %11 = *(%10 + 0)
    parm %10
    call %11
    return
}

FUNC<_Main.trueMain> {
    %2 = call _A._new
    %1 = %2
    parm 8
    %4 = call _Alloc
    %3 = FUNC<_lambda_22_19>
    *(%4 + 0) = %3
    *(%4 + 4) = %0
    *(%0 + 8) = %4
    %5 = *(%0 + 8)
    %6 = *(%5 + 0)
    parm %5
    parm 3
    %7 = call %6
    parm %7
    call _PrintInt
    *(%0 + 4) = 4
    parm 8
    %13 = call _Alloc
    %9 = FUNC<_f_25_23>
    *(%13 + 0) = %9
    *(%13 + 4) = %0
    %8 = %13
    %14 = *(%8 + 0)
    parm %8
    parm 3
    %15 = call %14
    parm %15
    call _PrintInt
    parm 4
    %19 = call _Alloc
    %17 = FUNC<_sf_27_23>
    *(%19 + 0) = %17
    %16 = %19
    %20 = *(%16 + 0)
    parm %16
    parm 3
    %21 = call %20
    parm %21
    call _PrintInt
    %23 = *(%0 + 8)
    %22 = %23
    %24 = *(%22 + 0)
    parm %22
    parm 3
    %25 = call %24
    parm %25
    call _PrintInt
    parm 8
    %31 = call _Alloc
    %27 = FUNC<_f_32_28>
    *(%31 + 0) = %27
    *(%31 + 4) = %0
    %26 = %31
    %32 = *(%26 + 0)
    parm %26
    parm 3
    %33 = call %32
    parm %33
    call _PrintInt
    parm 4
    %37 = call _Alloc
    %35 = FUNC<_sf_34_28>
    *(%37 + 0) = %35
    %34 = %37
    %38 = *(%34 + 0)
    parm %34
    parm 3
    %39 = call %38
    parm %39
    call _PrintInt
    %41 = *(%0 + 8)
    %40 = %41
    %42 = *(%40 + 0)
    parm %40
    parm 3
    %43 = call %42
    parm %43
    call _PrintInt
    parm 4
    %47 = call _Alloc
    %45 = FUNC<_sf_39_28>
    *(%47 + 0) = %45
    %44 = %47
    %48 = *(%44 + 0)
    parm %44
    parm 3
    %49 = call %48
    parm %49
    call _PrintInt
    parm 8
    %55 = call _Alloc
    %51 = FUNC<_f_41_26>
    *(%55 + 0) = %51
    *(%55 + 4) = %1
    %50 = %55
    %56 = *(%50 + 0)
    parm %50
    parm 3
    %57 = call %56
    parm %57
    call _PrintInt
    parm 4
    %61 = call _Alloc
    %59 = FUNC<_sf_43_26>
    *(%61 + 0) = %59
    %58 = %61
    %62 = *(%58 + 0)
    parm %58
    parm 3
    %63 = call %62
    parm %63
    call _PrintInt
    parm 4
    %67 = call _Alloc
    %65 = FUNC<_sf_46_26>
    *(%67 + 0) = %65
    %64 = %67
    %68 = *(%64 + 0)
    parm %64
    parm 3
    %69 = call %68
    parm %69
    call _PrintInt
    return
}

FUNC<_sf_15_23> {
    parm %1
    %2 = call _Main.sf
    return %2
}

FUNC<_trueMain_17_20> {
    %8 = *(%0 + 4)
    parm %8
    %9 = *(%8 + 0)
    %9 = *(%9 + 12)
    call %9
    return
}

FUNC<_lambda_22_19> {
    %2 = (%1 * 2)
    return %2
}

FUNC<_f_25_23> {
    %11 = *(%0 + 4)
    parm %11
    parm %1
    %12 = *(%11 + 0)
    %12 = *(%12 + 8)
    %10 = call %12
    return %10
}

FUNC<_sf_27_23> {
    parm %1
    %18 = call _Main.sf
    return %18
}

FUNC<_f_32_28> {
    %29 = *(%0 + 4)
    parm %29
    parm %1
    %30 = *(%29 + 0)
    %30 = *(%30 + 8)
    %28 = call %30
    return %28
}

FUNC<_sf_34_28> {
    parm %1
    %36 = call _Main.sf
    return %36
}

FUNC<_sf_39_28> {
    parm %1
    %46 = call _Main.sf
    return %46
}

FUNC<_f_41_26> {
    %53 = *(%0 + 4)
    parm %53
    parm %1
    %54 = *(%53 + 0)
    %54 = *(%54 + 8)
    %52 = call %54
    return %52
}

FUNC<_sf_43_26> {
    parm %1
    %60 = call _A.sf
    return %60
}

FUNC<_sf_46_26> {
    parm %1
    %66 = call _A.sf
    return %66
}

