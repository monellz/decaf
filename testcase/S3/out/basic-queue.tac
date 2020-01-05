VTBL<_QueueItem> {
    0
    "QueueItem"
    FUNC<_QueueItem.Init>
    FUNC<_QueueItem.GetData>
    FUNC<_QueueItem.GetNext>
    FUNC<_QueueItem.GetPrev>
    FUNC<_QueueItem.SetNext>
    FUNC<_QueueItem.SetPrev>
}

VTBL<_Queue> {
    0
    "Queue"
    FUNC<_Queue.Init>
    FUNC<_Queue.EnQueue>
    FUNC<_Queue.DeQueue>
}

VTBL<_Main> {
    0
    "Main"
}

FUNC<_QueueItem._new> {
    parm 16
    %0 = call _Alloc
    %1 = VTBL<_QueueItem>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    *(%0 + 8) = 0
    *(%0 + 12) = 0
    return %0
}

FUNC<_Queue._new> {
    parm 12
    %0 = call _Alloc
    %1 = VTBL<_Queue>
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

FUNC<_QueueItem.Init> {
    *(%0 + 4) = %1
    *(%0 + 8) = %2
    *(%2 + 12) = %0
    *(%0 + 12) = %3
    *(%3 + 8) = %0
    return
}

FUNC<_QueueItem.GetData> {
    %1 = *(%0 + 4)
    return %1
}

FUNC<_QueueItem.GetNext> {
    %1 = *(%0 + 8)
    return %1
}

FUNC<_QueueItem.GetPrev> {
    %1 = *(%0 + 12)
    return %1
}

FUNC<_QueueItem.SetNext> {
    *(%0 + 8) = %1
    return
}

FUNC<_QueueItem.SetPrev> {
    *(%0 + 12) = %1
    return
}

FUNC<_Queue.Init> {
    %1 = call _QueueItem._new
    *(%0 + 8) = %1
    %3 = *(%0 + 8)
    parm 8
    %6 = call _Alloc
    %2 = FUNC<_Init_36_19>
    *(%6 + 0) = %2
    *(%6 + 4) = %3
    %8 = *(%0 + 8)
    %9 = *(%0 + 8)
    %7 = *(%6 + 0)
    parm %6
    parm 0
    parm %8
    parm %9
    call %7
    return
}

FUNC<_Queue.EnQueue> {
    %2 = 0
    %3 = call _QueueItem._new
    %2 = %3
    parm 8
    %7 = call _Alloc
    %4 = FUNC<_Init_41_14>
    *(%7 + 0) = %4
    *(%7 + 4) = %2
    %11 = *(%0 + 8)
    parm 8
    %14 = call _Alloc
    %9 = FUNC<_GetNext_41_32>
    *(%14 + 0) = %9
    *(%14 + 4) = %11
    %15 = *(%14 + 0)
    parm %14
    %16 = call %15
    %17 = *(%0 + 8)
    %8 = *(%7 + 0)
    parm %7
    parm %1
    parm %16
    parm %17
    call %8
    return
}

FUNC<_Queue.DeQueue> {
    %1 = 0
    %4 = *(%0 + 8)
    parm 8
    %7 = call _Alloc
    %2 = FUNC<_GetPrev_45_23>
    *(%7 + 0) = %2
    *(%7 + 4) = %4
    %8 = *(%7 + 0)
    parm %7
    %9 = call %8
    %10 = *(%0 + 8)
    %11 = (%9 == %10)
    if (%11 == 0) branch %0
    %12 = "Queue Is Empty"
    parm %12
    call _PrintString
    return 0
    branch %1
    %0:
    %13 = 0
    %16 = *(%0 + 8)
    parm 8
    %19 = call _Alloc
    %14 = FUNC<_GetPrev_50_30>
    *(%19 + 0) = %14
    *(%19 + 4) = %16
    %20 = *(%19 + 0)
    parm %19
    %21 = call %20
    %13 = %21
    parm 8
    %26 = call _Alloc
    %22 = FUNC<_GetData_51_24>
    *(%26 + 0) = %22
    *(%26 + 4) = %13
    %27 = *(%26 + 0)
    parm %26
    %28 = call %27
    %1 = %28
    parm 8
    %34 = call _Alloc
    %30 = FUNC<_GetPrev_52_18>
    *(%34 + 0) = %30
    *(%34 + 4) = %13
    %35 = *(%34 + 0)
    parm %34
    %36 = call %35
    parm 8
    %39 = call _Alloc
    %29 = FUNC<_SetNext_52_28>
    *(%39 + 0) = %29
    *(%39 + 4) = %36
    parm 8
    %45 = call _Alloc
    %41 = FUNC<_GetNext_52_41>
    *(%45 + 0) = %41
    *(%45 + 4) = %13
    %46 = *(%45 + 0)
    parm %45
    %47 = call %46
    %40 = *(%39 + 0)
    parm %39
    parm %47
    call %40
    parm 8
    %53 = call _Alloc
    %49 = FUNC<_GetNext_53_18>
    *(%53 + 0) = %49
    *(%53 + 4) = %13
    %54 = *(%53 + 0)
    parm %53
    %55 = call %54
    parm 8
    %58 = call _Alloc
    %48 = FUNC<_SetPrev_53_28>
    *(%58 + 0) = %48
    *(%58 + 4) = %55
    parm 8
    %64 = call _Alloc
    %60 = FUNC<_GetPrev_53_41>
    *(%64 + 0) = %60
    *(%64 + 4) = %13
    %65 = *(%64 + 0)
    parm %64
    %66 = call %65
    %59 = *(%58 + 0)
    parm %58
    parm %66
    call %59
    %1:
    return %1
}

FUNC<main> {
    %0 = 0
    %1 = 0
    %2 = call _Queue._new
    %0 = %2
    parm 8
    %6 = call _Alloc
    %3 = FUNC<_Init_64_11>
    *(%6 + 0) = %3
    *(%6 + 4) = %0
    %7 = *(%6 + 0)
    parm %6
    call %7
    %1 = 0
    branch %0
    %1:
    parm 8
    %11 = call _Alloc
    %8 = FUNC<_EnQueue_66_15>
    *(%11 + 0) = %8
    *(%11 + 4) = %0
    %12 = *(%11 + 0)
    parm %11
    parm %1
    call %12
    %13 = (%1 + 1)
    %1 = %13
    %0:
    %14 = (%1 < 10)
    if (%14 != 0) branch %1
    %2:
    %1 = 0
    branch %3
    %4:
    parm 8
    %19 = call _Alloc
    %15 = FUNC<_DeQueue_69_21>
    *(%19 + 0) = %15
    *(%19 + 4) = %0
    %20 = *(%19 + 0)
    parm %19
    %21 = call %20
    parm %21
    call _PrintInt
    %22 = " "
    parm %22
    call _PrintString
    %23 = (%1 + 1)
    %1 = %23
    %3:
    %24 = (%1 < 4)
    if (%24 != 0) branch %4
    %5:
    %25 = "\n"
    parm %25
    call _PrintString
    %1 = 0
    branch %6
    %7:
    parm 8
    %29 = call _Alloc
    %26 = FUNC<_EnQueue_73_15>
    *(%29 + 0) = %26
    *(%29 + 4) = %0
    %30 = *(%29 + 0)
    parm %29
    parm %1
    call %30
    %31 = (%1 + 1)
    %1 = %31
    %6:
    %32 = (%1 < 10)
    if (%32 != 0) branch %7
    %8:
    %1 = 0
    branch %9
    %10:
    parm 8
    %37 = call _Alloc
    %33 = FUNC<_DeQueue_75_21>
    *(%37 + 0) = %33
    *(%37 + 4) = %0
    %38 = *(%37 + 0)
    parm %37
    %39 = call %38
    parm %39
    call _PrintInt
    %40 = " "
    parm %40
    call _PrintString
    %41 = (%1 + 1)
    %1 = %41
    %9:
    %42 = (%1 < 17)
    if (%42 != 0) branch %10
    %11:
    %43 = "\n"
    parm %43
    call _PrintString
    return
}

FUNC<_Init_36_19> {
    %4 = *(%0 + 4)
    parm %4
    parm %1
    parm %2
    parm %3
    %5 = *(%4 + 0)
    %5 = *(%5 + 8)
    call %5
    return
}

FUNC<_Init_41_14> {
    %5 = *(%0 + 4)
    parm %5
    parm %1
    parm %2
    parm %3
    %6 = *(%5 + 0)
    %6 = *(%6 + 8)
    call %6
    return
}

FUNC<_GetNext_41_32> {
    %12 = *(%0 + 4)
    parm %12
    %13 = *(%12 + 0)
    %13 = *(%13 + 16)
    %10 = call %13
    return %10
}

FUNC<_GetPrev_45_23> {
    %5 = *(%0 + 4)
    parm %5
    %6 = *(%5 + 0)
    %6 = *(%6 + 20)
    %3 = call %6
    return %3
}

FUNC<_GetPrev_50_30> {
    %17 = *(%0 + 4)
    parm %17
    %18 = *(%17 + 0)
    %18 = *(%18 + 20)
    %15 = call %18
    return %15
}

FUNC<_GetData_51_24> {
    %24 = *(%0 + 4)
    parm %24
    %25 = *(%24 + 0)
    %25 = *(%25 + 12)
    %23 = call %25
    return %23
}

FUNC<_GetPrev_52_18> {
    %32 = *(%0 + 4)
    parm %32
    %33 = *(%32 + 0)
    %33 = *(%33 + 20)
    %31 = call %33
    return %31
}

FUNC<_SetNext_52_28> {
    %37 = *(%0 + 4)
    parm %37
    parm %1
    %38 = *(%37 + 0)
    %38 = *(%38 + 24)
    call %38
    return
}

FUNC<_GetNext_52_41> {
    %43 = *(%0 + 4)
    parm %43
    %44 = *(%43 + 0)
    %44 = *(%44 + 16)
    %42 = call %44
    return %42
}

FUNC<_GetNext_53_18> {
    %51 = *(%0 + 4)
    parm %51
    %52 = *(%51 + 0)
    %52 = *(%52 + 16)
    %50 = call %52
    return %50
}

FUNC<_SetPrev_53_28> {
    %56 = *(%0 + 4)
    parm %56
    parm %1
    %57 = *(%56 + 0)
    %57 = *(%57 + 28)
    call %57
    return
}

FUNC<_GetPrev_53_41> {
    %62 = *(%0 + 4)
    parm %62
    %63 = *(%62 + 0)
    %63 = *(%63 + 20)
    %61 = call %63
    return %61
}

FUNC<_Init_64_11> {
    %4 = *(%0 + 4)
    parm %4
    %5 = *(%4 + 0)
    %5 = *(%5 + 8)
    call %5
    return
}

FUNC<_EnQueue_66_15> {
    %9 = *(%0 + 4)
    parm %9
    parm %1
    %10 = *(%9 + 0)
    %10 = *(%10 + 12)
    call %10
    return
}

FUNC<_DeQueue_69_21> {
    %17 = *(%0 + 4)
    parm %17
    %18 = *(%17 + 0)
    %18 = *(%18 + 16)
    %16 = call %18
    return %16
}

FUNC<_EnQueue_73_15> {
    %27 = *(%0 + 4)
    parm %27
    parm %1
    %28 = *(%27 + 0)
    %28 = *(%28 + 12)
    call %28
    return
}

FUNC<_DeQueue_75_21> {
    %35 = *(%0 + 4)
    parm %35
    %36 = *(%35 + 0)
    %36 = *(%36 + 16)
    %34 = call %36
    return %34
}

