# Makemake

> 鸟神星是太阳系内已知的第四大矮行星，亦是经典柯伊伯带天体中最大的两颗之一。

`raft crate` 从 iris 中抽离。

`iris` 中存储数据的结构为 `HashMap<String, String> `，在消息队列的场景中，`raft` 被用于实现分布式锁和队列数据结构，因此需要对
`log entry` 和 `data` 以及对应处理逻辑进行一些修改。

> uwu 坏了写得乱七八糟的，呜呜要被骂了