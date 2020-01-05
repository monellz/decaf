VTBL<_A> {
    0
    "A"
    FUNC<_A.a>
    FUNC<_A.b>
    FUNC<_A.c>
    FUNC<_A.d>
}

VTBL<_B> {
    VTBL<_A>
    "B"
    FUNC<_B.a>
    FUNC<_B.b>
    FUNC<_A.c>
    FUNC<_A.d>
    FUNC<_B.e>
    FUNC<_B.f>
}

VTBL<_C> {
    VTBL<_B>
    "C"
    FUNC<_C.a>
    FUNC<_B.b>
    FUNC<_C.c>
    FUNC<_C.d>
    FUNC<_C.e>
    FUNC<_C.f>
}

VTBL<_D> {
    VTBL<_C>
    "D"
    FUNC<_C.a>
    FUNC<_B.b>
    FUNC<_D.c>
    FUNC<_D.d>
    FUNC<_D.e>
    FUNC<_D.f>
    FUNC<_D.g>
    FUNC<_D.h>
}

VTBL<_E> {
    VTBL<_C>
    "E"
    FUNC<_E.a>
    FUNC<_B.b>
    FUNC<_C.c>
    FUNC<_E.d>
    FUNC<_C.e>
    FUNC<_C.f>
    FUNC<_E.g>
}

VTBL<_F> {
    VTBL<_D>
    "F"
    FUNC<_C.a>
    FUNC<_B.b>
    FUNC<_D.c>
    FUNC<_D.d>
    FUNC<_D.e>
    FUNC<_D.f>
    FUNC<_D.g>
    FUNC<_F.h>
}

VTBL<_Main> {
    VTBL<_D>
    "Main"
    FUNC<_Main.a>
    FUNC<_B.b>
    FUNC<_D.c>
    FUNC<_Main.d>
    FUNC<_D.e>
    FUNC<_D.f>
    FUNC<_Main.g>
    FUNC<_Main.h>
}

FUNC<_A._new> {
    parm 8
    %0 = call _Alloc
    %1 = VTBL<_A>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    return %0
}

FUNC<_B._new> {
    parm 12
    %0 = call _Alloc
    %1 = VTBL<_B>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    *(%0 + 8) = 0
    return %0
}

FUNC<_C._new> {
    parm 12
    %0 = call _Alloc
    %1 = VTBL<_C>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    *(%0 + 8) = 0
    return %0
}

FUNC<_D._new> {
    parm 12
    %0 = call _Alloc
    %1 = VTBL<_D>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    *(%0 + 8) = 0
    return %0
}

FUNC<_E._new> {
    parm 12
    %0 = call _Alloc
    %1 = VTBL<_E>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    *(%0 + 8) = 0
    return %0
}

FUNC<_F._new> {
    parm 12
    %0 = call _Alloc
    %1 = VTBL<_F>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    *(%0 + 8) = 0
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

FUNC<_A.a> {
    return
}

FUNC<_A.b> {
    return
}

FUNC<_A.c> {
    return
}

FUNC<_A.d> {
    return
}

FUNC<_B.a> {
    return
}

FUNC<_B.b> {
    %1 = "B.b()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_B.e> {
    return
}

FUNC<_B.f> {
    %1 = "B.f()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_C.a> {
    %1 = "C.a()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_C.c> {
    %1 = "C.c()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_C.d> {
    %1 = "C.d()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_C.e> {
    %1 = "C.e()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_C.f> {
    %1 = "C.f()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_D.c> {
    %1 = "D.c()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_D.d> {
    %1 = "D.d()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_D.e> {
    %1 = "D.e()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_D.f> {
    %1 = "D.f()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_D.g> {
    return
}

FUNC<_D.h> {
}

FUNC<_E.a> {
    %1 = "E.a()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_E.d> {
    %1 = "E.d()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_E.g> {
    %3 = "E.g()\n"
    parm %3
    call _PrintString
    return
}

FUNC<_F.h> {
}

FUNC<_Main.a> {
    %1 = "Main.a()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_Main.d> {
    %1 = "Main.d()\n"
    parm %1
    call _PrintString
    return
}

FUNC<_Main.g> {
    %3 = "Main.g()\n"
    parm %3
    call _PrintString
    return
}

FUNC<_Main.h> {
    %1 = "Main.h()\n"
    parm %1
    call _PrintString
    %2 = call _Main._new
    return %2
}

FUNC<main> {
    %1 = call _C._new
    %0 = %1
    parm 8
    %5 = call _Alloc
    %2 = FUNC<_a_53_11>
    *(%5 + 0) = %2
    *(%5 + 4) = %0
    %6 = *(%5 + 0)
    parm %5
    call %6
    parm 8
    %10 = call _Alloc
    %7 = FUNC<_b_54_11>
    *(%10 + 0) = %7
    *(%10 + 4) = %0
    %11 = *(%10 + 0)
    parm %10
    call %11
    parm 8
    %15 = call _Alloc
    %12 = FUNC<_c_55_11>
    *(%15 + 0) = %12
    *(%15 + 4) = %0
    %16 = *(%15 + 0)
    parm %15
    call %16
    parm 8
    %20 = call _Alloc
    %17 = FUNC<_d_56_11>
    *(%20 + 0) = %17
    *(%20 + 4) = %0
    %21 = *(%20 + 0)
    parm %20
    call %21
    parm 8
    %25 = call _Alloc
    %22 = FUNC<_e_57_11>
    *(%25 + 0) = %22
    *(%25 + 4) = %0
    %26 = *(%25 + 0)
    parm %25
    call %26
    parm 8
    %30 = call _Alloc
    %27 = FUNC<_f_58_11>
    *(%30 + 0) = %27
    *(%30 + 4) = %0
    %31 = *(%30 + 0)
    parm %30
    call %31
    %33 = call _E._new
    %32 = %33
    parm 8
    %37 = call _Alloc
    %34 = FUNC<_a_61_11>
    *(%37 + 0) = %34
    *(%37 + 4) = %32
    %38 = *(%37 + 0)
    parm %37
    call %38
    parm 8
    %42 = call _Alloc
    %39 = FUNC<_b_62_11>
    *(%42 + 0) = %39
    *(%42 + 4) = %32
    %43 = *(%42 + 0)
    parm %42
    call %43
    parm 8
    %47 = call _Alloc
    %44 = FUNC<_c_63_11>
    *(%47 + 0) = %44
    *(%47 + 4) = %32
    %48 = *(%47 + 0)
    parm %47
    call %48
    parm 8
    %52 = call _Alloc
    %49 = FUNC<_d_64_11>
    *(%52 + 0) = %49
    *(%52 + 4) = %32
    %53 = *(%52 + 0)
    parm %52
    call %53
    parm 8
    %57 = call _Alloc
    %54 = FUNC<_e_65_11>
    *(%57 + 0) = %54
    *(%57 + 4) = %32
    %58 = *(%57 + 0)
    parm %57
    call %58
    parm 8
    %62 = call _Alloc
    %59 = FUNC<_f_66_11>
    *(%62 + 0) = %59
    *(%62 + 4) = %32
    %63 = *(%62 + 0)
    parm %62
    call %63
    parm 8
    %67 = call _Alloc
    %64 = FUNC<_g_67_11>
    *(%67 + 0) = %64
    *(%67 + 4) = %32
    %68 = *(%67 + 0)
    parm %67
    parm 1
    parm 1
    call %68
    %70 = call _Main._new
    %69 = %70
    parm 8
    %74 = call _Alloc
    %71 = FUNC<_a_69_11>
    *(%74 + 0) = %71
    *(%74 + 4) = %69
    %75 = *(%74 + 0)
    parm %74
    call %75
    parm 8
    %79 = call _Alloc
    %76 = FUNC<_b_70_11>
    *(%79 + 0) = %76
    *(%79 + 4) = %69
    %80 = *(%79 + 0)
    parm %79
    call %80
    parm 8
    %84 = call _Alloc
    %81 = FUNC<_c_71_11>
    *(%84 + 0) = %81
    *(%84 + 4) = %69
    %85 = *(%84 + 0)
    parm %84
    call %85
    parm 8
    %89 = call _Alloc
    %86 = FUNC<_d_72_11>
    *(%89 + 0) = %86
    *(%89 + 4) = %69
    %90 = *(%89 + 0)
    parm %89
    call %90
    parm 8
    %94 = call _Alloc
    %91 = FUNC<_e_73_11>
    *(%94 + 0) = %91
    *(%94 + 4) = %69
    %95 = *(%94 + 0)
    parm %94
    call %95
    parm 8
    %99 = call _Alloc
    %96 = FUNC<_f_74_11>
    *(%99 + 0) = %96
    *(%99 + 4) = %69
    %100 = *(%99 + 0)
    parm %99
    call %100
    parm 8
    %104 = call _Alloc
    %101 = FUNC<_g_75_11>
    *(%104 + 0) = %101
    *(%104 + 4) = %69
    %105 = *(%104 + 0)
    parm %104
    parm 1
    parm 1
    call %105
    parm 8
    %110 = call _Alloc
    %106 = FUNC<_h_76_11>
    *(%110 + 0) = %106
    *(%110 + 4) = %69
    %111 = *(%110 + 0)
    parm %110
    %112 = call %111
    return
}

FUNC<_a_53_11> {
    %3 = *(%0 + 4)
    parm %3
    %4 = *(%3 + 0)
    %4 = *(%4 + 8)
    call %4
    return
}

FUNC<_b_54_11> {
    %8 = *(%0 + 4)
    parm %8
    %9 = *(%8 + 0)
    %9 = *(%9 + 12)
    call %9
    return
}

FUNC<_c_55_11> {
    %13 = *(%0 + 4)
    parm %13
    %14 = *(%13 + 0)
    %14 = *(%14 + 16)
    call %14
    return
}

FUNC<_d_56_11> {
    %18 = *(%0 + 4)
    parm %18
    %19 = *(%18 + 0)
    %19 = *(%19 + 20)
    call %19
    return
}

FUNC<_e_57_11> {
    %23 = *(%0 + 4)
    parm %23
    %24 = *(%23 + 0)
    %24 = *(%24 + 24)
    call %24
    return
}

FUNC<_f_58_11> {
    %28 = *(%0 + 4)
    parm %28
    %29 = *(%28 + 0)
    %29 = *(%29 + 28)
    call %29
    return
}

FUNC<_a_61_11> {
    %35 = *(%0 + 4)
    parm %35
    %36 = *(%35 + 0)
    %36 = *(%36 + 8)
    call %36
    return
}

FUNC<_b_62_11> {
    %40 = *(%0 + 4)
    parm %40
    %41 = *(%40 + 0)
    %41 = *(%41 + 12)
    call %41
    return
}

FUNC<_c_63_11> {
    %45 = *(%0 + 4)
    parm %45
    %46 = *(%45 + 0)
    %46 = *(%46 + 16)
    call %46
    return
}

FUNC<_d_64_11> {
    %50 = *(%0 + 4)
    parm %50
    %51 = *(%50 + 0)
    %51 = *(%51 + 20)
    call %51
    return
}

FUNC<_e_65_11> {
    %55 = *(%0 + 4)
    parm %55
    %56 = *(%55 + 0)
    %56 = *(%56 + 24)
    call %56
    return
}

FUNC<_f_66_11> {
    %60 = *(%0 + 4)
    parm %60
    %61 = *(%60 + 0)
    %61 = *(%61 + 28)
    call %61
    return
}

FUNC<_g_67_11> {
    %65 = *(%0 + 4)
    parm %65
    parm %1
    parm %2
    %66 = *(%65 + 0)
    %66 = *(%66 + 32)
    call %66
    return
}

FUNC<_a_69_11> {
    %72 = *(%0 + 4)
    parm %72
    %73 = *(%72 + 0)
    %73 = *(%73 + 8)
    call %73
    return
}

FUNC<_b_70_11> {
    %77 = *(%0 + 4)
    parm %77
    %78 = *(%77 + 0)
    %78 = *(%78 + 12)
    call %78
    return
}

FUNC<_c_71_11> {
    %82 = *(%0 + 4)
    parm %82
    %83 = *(%82 + 0)
    %83 = *(%83 + 16)
    call %83
    return
}

FUNC<_d_72_11> {
    %87 = *(%0 + 4)
    parm %87
    %88 = *(%87 + 0)
    %88 = *(%88 + 20)
    call %88
    return
}

FUNC<_e_73_11> {
    %92 = *(%0 + 4)
    parm %92
    %93 = *(%92 + 0)
    %93 = *(%93 + 24)
    call %93
    return
}

FUNC<_f_74_11> {
    %97 = *(%0 + 4)
    parm %97
    %98 = *(%97 + 0)
    %98 = *(%98 + 28)
    call %98
    return
}

FUNC<_g_75_11> {
    %102 = *(%0 + 4)
    parm %102
    parm %1
    parm %2
    %103 = *(%102 + 0)
    %103 = *(%103 + 32)
    call %103
    return
}

FUNC<_h_76_11> {
    %108 = *(%0 + 4)
    parm %108
    %109 = *(%108 + 0)
    %109 = *(%109 + 36)
    %107 = call %109
    return %107
}

