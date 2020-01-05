VTBL<_Complex> {
    0
    "Complex"
    FUNC<_Complex.abs2>
}

VTBL<_Main> {
    VTBL<_Complex>
    "Main"
    FUNC<_Complex.abs2>
}

FUNC<_Complex._new> {
    parm 12
    %0 = call _Alloc
    %1 = VTBL<_Complex>
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

FUNC<_Complex.make> {
    %3 = call _Complex._new
    %4 = (%0 % 32768)
    *(%3 + 4) = %4
    %5 = (%1 % 32768)
    *(%3 + 8) = %5
    return %3
}

FUNC<_Complex.add> {
    %3 = *(%0 + 4)
    %4 = *(%1 + 4)
    %5 = (%3 + %4)
    %6 = *(%0 + 8)
    %7 = *(%1 + 8)
    %8 = (%6 + %7)
    parm %5
    parm %8
    %2 = call _Complex.make
    return %2
}

FUNC<_Complex.sub> {
    %3 = *(%0 + 4)
    %4 = *(%1 + 4)
    %5 = (%3 - %4)
    %6 = *(%0 + 8)
    %7 = *(%1 + 8)
    %8 = (%6 - %7)
    parm %5
    parm %8
    %2 = call _Complex.make
    return %2
}

FUNC<_Complex.mul> {
    %19 = *(%0 + 4)
    %22 = *(%1 + 4)
    %5 = (%19 * %22)
    %21 = *(%0 + 8)
    %20 = *(%1 + 8)
    %8 = (%21 * %20)
    %9 = (%5 - %8)
    %10 = (%9 / 4096)
    %13 = (%19 * %20)
    %16 = (%21 * %22)
    %17 = (%13 + %16)
    %18 = (%17 / 4096)
    parm %10
    parm %18
    %2 = call _Complex.make
    return %2
}

FUNC<_Complex.abs2> {
    %8 = *(%0 + 4)
    %3 = (%8 * %8)
    %9 = *(%0 + 8)
    %6 = (%9 * %9)
    %7 = (%3 + %6)
    return %7
}

FUNC<main> {
    parm 208
    %13 = call _Alloc
    %11 = (%13 + 208)
    %13 = (%13 + 4)
    branch %3
    %2:
    %11 = (%11 - 4)
    *(%11 + 0) = 0
    %3:
    %10 = (%11 == %13)
    if (%10 == 0) branch %2
    *(%13 - 4) = 51
    parm 208
    %18 = call _Alloc
    %16 = (%18 + 208)
    %18 = (%18 + 4)
    branch %7
    %6:
    %16 = (%16 - 4)
    *(%16 + 0) = 0
    %7:
    %15 = (%16 == %18)
    if (%15 == 0) branch %6
    *(%18 - 4) = 51
    %19 = 0
    branch %39
    %9:
    parm 208
    %23 = call _Alloc
    %21 = (%23 + 208)
    %23 = (%23 + 4)
    branch %12
    %11:
    %21 = (%21 - 4)
    *(%21 + 0) = 0
    %12:
    %20 = (%21 == %23)
    if (%20 == 0) branch %11
    *(%23 - 4) = 51
    %157 = *(%13 - 4)
    %155 = (%19 >= 0)
    %176 = (%19 < %157)
    %180 = (%155 && %176)
    if (%180 == 0) branch %15
    %156 = (%19 * 4)
    %181 = (%156 + %13)
    *(%181 + 0) = %23
    branch %16
    %15:
    %28 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %28
    call _PrintString
    call _Halt
    %16:
    parm 208
    %32 = call _Alloc
    %30 = (%32 + 208)
    %32 = (%32 + 4)
    branch %19
    %18:
    %30 = (%30 - 4)
    *(%30 + 0) = 0
    %19:
    %29 = (%30 == %32)
    if (%29 == 0) branch %18
    *(%32 - 4) = 51
    %160 = *(%18 - 4)
    %177 = (%19 < %160)
    %182 = (%155 && %177)
    if (%182 == 0) branch %22
    %183 = (%156 + %18)
    *(%183 + 0) = %32
    branch %23
    %22:
    %37 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %37
    call _PrintString
    call _Halt
    %23:
    %38 = 0
    branch %37
    %24:
    %40 = (%38 * 327)
    %41 = (-8192 + %40)
    %42 = (%19 * 327)
    %43 = (-8192 + %42)
    parm %41
    parm %43
    %39 = call _Complex.make
    if (%180 == 0) branch %26
    %48 = *(%181 + 0)
    branch %27
    %26:
    %49 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %49
    call _PrintString
    call _Halt
    %27:
    %51 = *(%48 - 4)
    %163 = (%38 >= 0)
    %52 = (%38 < %51)
    %50 = (%163 && %52)
    if (%50 == 0) branch %29
    %164 = (%38 * 4)
    %53 = (%164 + %48)
    *(%53 + 0) = %39
    branch %30
    %29:
    %54 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %54
    call _PrintString
    call _Halt
    %30:
    %55 = call _Complex._new
    if (%182 == 0) branch %32
    %60 = *(%183 + 0)
    branch %33
    %32:
    %61 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %61
    call _PrintString
    call _Halt
    %33:
    %63 = *(%60 - 4)
    %64 = (%38 < %63)
    %62 = (%163 && %64)
    if (%62 == 0) branch %35
    %65 = (%164 + %60)
    *(%65 + 0) = %55
    branch %36
    %35:
    %66 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %66
    call _PrintString
    call _Halt
    %36:
    %67 = (%38 + 1)
    %38 = %67
    %37:
    %68 = (%38 < 51)
    if (%68 != 0) branch %24
    %69 = (%19 + 1)
    %19 = %69
    %39:
    %70 = (%19 < 51)
    if (%70 != 0) branch %9
    %71 = 0
    branch %67
    %41:
    %72 = 0
    branch %65
    %42:
    %73 = 0
    branch %63
    %43:
    %170 = *(%18 - 4)
    %166 = (%72 >= 0)
    %178 = (%72 < %170)
    %184 = (%166 && %178)
    if (%184 == 0) branch %45
    %167 = (%72 * 4)
    %179 = (%167 + %18)
    %185 = *(%179 + 0)
    branch %46
    %45:
    %80 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %80
    call _PrintString
    call _Halt
    %46:
    %186 = *(%185 - 4)
    %168 = (%73 >= 0)
    %188 = (%73 < %186)
    %189 = (%168 && %188)
    if (%189 == 0) branch %48
    %169 = (%73 * 4)
    %187 = (%169 + %185)
    %85 = *(%187 + 0)
    branch %49
    %48:
    %86 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %86
    call _PrintString
    call _Halt
    %49:
    parm %85
    %88 = *(%85 + 0)
    %88 = *(%88 + 8)
    %87 = call %88
    %91 = (%87 < 67108864)
    if (%91 == 0) branch %62
    parm %85
    parm %85
    %93 = call _Complex.mul
    %95 = *(%13 - 4)
    %96 = (%72 < %95)
    %94 = (%166 && %96)
    if (%94 == 0) branch %52
    %97 = (%167 + %13)
    %98 = *(%97 + 0)
    branch %53
    %52:
    %99 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %99
    call _PrintString
    call _Halt
    %53:
    %101 = *(%98 - 4)
    %102 = (%73 < %101)
    %100 = (%168 && %102)
    if (%100 == 0) branch %55
    %103 = (%169 + %98)
    %104 = *(%103 + 0)
    branch %56
    %55:
    %105 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %105
    call _PrintString
    call _Halt
    %56:
    parm %93
    parm %104
    %92 = call _Complex.add
    if (%184 == 0) branch %58
    branch %59
    %58:
    %111 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %111
    call _PrintString
    call _Halt
    %59:
    if (%189 == 0) branch %61
    *(%187 + 0) = %92
    branch %62
    %61:
    %116 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %116
    call _PrintString
    call _Halt
    %62:
    %117 = (%73 + 1)
    %73 = %117
    %63:
    %118 = (%73 < 51)
    if (%118 != 0) branch %43
    %119 = (%72 + 1)
    %72 = %119
    %65:
    %120 = (%72 < 51)
    if (%120 != 0) branch %42
    %121 = (%71 + 1)
    %71 = %121
    %67:
    %122 = (%71 < 20)
    if (%122 != 0) branch %41
    %123 = 0
    branch %82
    %69:
    %124 = 0
    branch %80
    %70:
    %127 = *(%18 - 4)
    %126 = (%123 >= 0)
    %128 = (%123 < %127)
    %126 = (%126 && %128)
    if (%126 == 0) branch %72
    %129 = (%123 * 4)
    %129 = (%129 + %18)
    %130 = *(%129 + 0)
    branch %73
    %72:
    %131 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %131
    call _PrintString
    call _Halt
    %73:
    %133 = *(%130 - 4)
    %132 = (%124 >= 0)
    %134 = (%124 < %133)
    %132 = (%132 && %134)
    if (%132 == 0) branch %75
    %135 = (%124 * 4)
    %135 = (%135 + %130)
    %136 = *(%135 + 0)
    branch %76
    %75:
    %137 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %137
    call _PrintString
    call _Halt
    %76:
    parm %136
    %138 = *(%136 + 0)
    %138 = *(%138 + 8)
    %125 = call %138
    %141 = (%125 < 67108864)
    if (%141 == 0) branch %78
    %142 = "**"
    parm %142
    call _PrintString
    branch %79
    %78:
    %143 = "  "
    parm %143
    call _PrintString
    %79:
    %144 = (%124 + 1)
    %124 = %144
    %80:
    %145 = (%124 < 51)
    if (%145 != 0) branch %70
    %146 = "\n"
    parm %146
    call _PrintString
    %147 = (%123 + 1)
    %123 = %147
    %82:
    %148 = (%123 < 51)
    if (%148 != 0) branch %69
    return
}

