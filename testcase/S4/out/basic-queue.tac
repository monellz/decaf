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
    %6 = *(%0 + 8)
    parm %6
    parm 0
    parm %6
    parm %6
    %5 = *(%6 + 0)
    %5 = *(%5 + 8)
    call %5
    return
}

FUNC<_Queue.EnQueue> {
    %3 = call _QueueItem._new
    %5 = *(%0 + 8)
    parm %5
    %6 = *(%5 + 0)
    %6 = *(%6 + 16)
    %4 = call %6
    %7 = *(%0 + 8)
    parm %3
    parm %1
    parm %4
    parm %7
    %8 = *(%3 + 0)
    %8 = *(%8 + 8)
    call %8
    return
}

FUNC<_Queue.DeQueue> {
    %3 = *(%0 + 8)
    parm %3
    %4 = *(%3 + 0)
    %4 = *(%4 + 20)
    %2 = call %4
    %24 = *(%0 + 8)
    %6 = (%2 == %24)
    if (%6 == 0) branch %2
    %7 = "Queue Is Empty"
    parm %7
    call _PrintString
    return 0
    %2:
    parm %24
    %11 = *(%24 + 0)
    %11 = *(%11 + 20)
    %9 = call %11
    parm %9
    %25 = *(%9 + 0)
    %13 = *(%25 + 12)
    %12 = call %13
    parm %9
    %30 = *(%25 + 16)
    %14 = call %30
    parm %9
    %29 = *(%25 + 20)
    %16 = call %29
    parm %16
    parm %14
    %18 = *(%16 + 0)
    %18 = *(%18 + 24)
    call %18
    parm %9
    %19 = call %29
    parm %9
    %21 = call %30
    parm %21
    parm %19
    %23 = *(%21 + 0)
    %23 = *(%23 + 28)
    call %23
    return %12
}

FUNC<main> {
    %2 = call _Queue._new
    parm %2
    %25 = *(%2 + 0)
    %24 = %25
    %23 = %25
    %22 = %25
    %3 = *(%25 + 8)
    call %3
    %1 = 0
    branch %2
    %1:
    parm %2
    parm %1
    %25 = %22
    %24 = %22
    %23 = %22
    %4 = *(%22 + 12)
    call %4
    %5 = (%1 + 1)
    %1 = %5
    %2:
    %6 = (%1 < 10)
    if (%6 != 0) branch %1
    %1 = 0
    branch %5
    %4:
    parm %2
    %25 = %23
    %24 = %23
    %8 = *(%23 + 16)
    %7 = call %8
    parm %7
    call _PrintInt
    %9 = " "
    parm %9
    call _PrintString
    %10 = (%1 + 1)
    %1 = %10
    %5:
    %11 = (%1 < 4)
    if (%11 != 0) branch %4
    %12 = "\n"
    parm %12
    call _PrintString
    %1 = 0
    branch %8
    %7:
    parm %2
    parm %1
    %25 = %24
    %13 = *(%24 + 12)
    call %13
    %14 = (%1 + 1)
    %1 = %14
    %8:
    %15 = (%1 < 10)
    if (%15 != 0) branch %7
    %1 = 0
    branch %11
    %10:
    parm %2
    %17 = *(%25 + 16)
    %16 = call %17
    parm %16
    call _PrintInt
    %18 = " "
    parm %18
    call _PrintString
    %19 = (%1 + 1)
    %1 = %19
    %11:
    %20 = (%1 < 17)
    if (%20 != 0) branch %10
    %21 = "\n"
    parm %21
    call _PrintString
    return
}

