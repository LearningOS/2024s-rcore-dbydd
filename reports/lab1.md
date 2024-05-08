## 编程部分

实现思路：在TaskControlBlock中加入计数器和开始时间戳，为外层的TaskInner新增fetch_task_info和inc_call_times方法，前者在task_info这个syscall的处理函数中调用，后者在所有syscall处理函数中调用，参数为对应的id。

## 问答题

1. 三个错误程序
   * ch2b_bad_address.rs : PageFault in application, bad addr = 0x0, bad instruction = 0x804003c4, kernel killed it. 简单描述：试图访问0x0地址，于是炸了
   * ch2b_bad_instructions与ch2b_bad_register: IllegalInstruction in application, kernel killed it.简单描述：sret是(S)模式用来返回(U)模式的指令,csr寄存器是(S)模式或更高才能访问的，因此报了非法操作。
   * sbi版本：使用仓库默认rustsbi-qemu
2. trap.S
   1. 查找risc-v寄存器表可知，a0保存的是函数的参数/返回值，观察os/src/task/context.rs可知，这里保存的是kernel stack 的sp，也就是栈底指针。
   2. t0-t2是临时变量寄存器，这里load的变量，查看L29，30，35可知，分别是sstatus(处理器当前操作状态),sepc(S模式返回地址)和sscratch(指向user stack的sp)
   3. x2寄存器的abi name是sp,也就是栈指针，必须要最后一个修改。同理,x4是thread pointer，指向当前线程。
   4. sscratch本身是寄存器地址，不变，但是其中的值会被写到sp中，然后sp中的值会被写到前者中。
   5. 最后一条sret，会触发运行环境切换。参考原文：`<S/U>`ret: returns from a trap *from* the `<s/u>` mode
   6. 这里是在进行user stack和kernel stack的切换，是L60的相反操作
   7. L38，call trap_handler之后

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > *无*
   >
2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > 反复查看手册确认题目要求算吗
   >
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
