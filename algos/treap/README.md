# rust怎么写多重引用的数据结构

safe rust怎么用来实现像dancing list/skip list或者循环链表这样的多引用的数据结构

- 能否通过一个主引用(Box<Node>), 多个副引用(可以获取mut引用的结构, 最好不是RefCell, 而是某种unsafe RefCell)
    - RefCell<Box<T>> ? 我尼玛裂开
- 自相矛盾, 那能否通过优雅的unsafe实现呢?
- 杀妈了, 二叉树之类的单独引用的还能用, 但是这个就简直感人
