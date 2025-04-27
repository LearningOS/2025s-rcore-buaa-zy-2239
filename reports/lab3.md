### lab3实验报告(ch5)
#### 实现功能总结
+ 实现了一个新的系统调用sys_spawn,它可以根据输入的文件路径创建一个新的子进程执行对应程序，且该子进程不像fork一样复制了父进程的地址空间。
+ 实现了stride调度算法。在TCBInner中拓展了stride、pass等相关字段，增加了sys_set_priority系统调用函数，并在Taskmanager中完成了stride调度算法的实现，实现了优先级高的进程可以分配到更多的运行事件的效果。
#### 问答作业
> stride 算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存 stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。实际情况是轮到 p1 执行吗？为什么？

不是轮到p1执行，因为会发生溢出，使得p1.stride=255,\
p2.stride=4,此时p1.stride=255大于p2.stride,使得接下来很长一段时间片都会由p2执行。
> 我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， 在不考虑溢出的情况下 , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。为什么？尝试简单说明（不要求严格证明）。

因为严格按照算法执行的话，每次时间片执行时，STRIDE_MIN进程stride的增加$\leq \frac{BigStride}{2}$,若多个进程同时进行
+ 那么若该进程stride增加后变为STRIDE_MAX进程，因为之前Stride小于倒数第二个进程，则STRIDE_MAX进程和STRDE_MIN进程Stride之差小于$\frac{BigStride}{2}$,
+ 若未变为MAX进程，则说明MAX进程和MIN进程Stride之差小于pass_min,进而小于$\frac{BIGSTRIDE}{2}$。

综上述，可以保证$STRIDE_MAX – STRIDE_MIN <= \frac{BigStride}{2}$
> 已知以上结论，考虑溢出的情况下，可以为 Stride 设计特别的比较器，让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 partial_cmp 函数，假设两个 Stride 永远不会相等。

首先，根据上述结论，我们可知，在不考虑溢出情况下，进程间的步数之差小于$\frac{BIGStride}{2}$,因此对于任何进程，均应当满足上述结论，若未满足上述结论，说明较小步长的进程必然发生了溢出，因此输出Greater，反之，输出Less。
```
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.0 as u8;
        let b = other.0 as u8;
        let diff = b.wrapping_sub(a);
        if diff < BigStride/2 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```
#### 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。