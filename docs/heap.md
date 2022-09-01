---
time: 2022/8/31
title: 二叉堆实现 in rust
---
# 二叉堆(Binary Heap)

## 引入

- **完全二叉树**: 若设二叉树的深度为*n*除第*n*层外，其它各层 (1～*n*−1) 的结点数都达到最大个数，第*n*层所有的结点都连续集中在最左边，这就是完全二叉树。
- **满二叉树**: 一个二叉树，如果每一个层的结点数都达到最大值，则这个二叉树就是满二叉树。也就是说，如果一个二叉树的层数为*n*，且结点总数是2*n*−1 ，则它就是满二叉树。

- **优先队列**: 堆(heap)又被称为**优先队列(priority queue)**。尽管名为优先队列，但堆并不是队列。回忆一下，在队列中，我们可以进行的限定操作是dequeue和enqueue。dequeue是按照进入队列的先后顺序来取出元素。而在堆中，我们不是按照元素进入队列的先后顺序取出元素的，而是按照元素的优先级取出元素。

- 堆的一个经典的实现是**完全二叉树(complete binary tree)**。这样实现的堆成为**二叉堆(binary heap)**。由于其它几种堆（二项式堆，斐波纳契堆等）用的较少，**一般将二叉堆就简称为堆。**

## 二叉堆特性

 1. 父结点的键值总是大于或等于（小于或等于）任何一个子节点的键值
 2. 每个结点的左子树和右子树都是一个二叉堆（都是最大堆或最小堆

## 实现方法

1. 添加节点: `insert()`
   - `swim()`
2. 提取最值: `extract()`
   - `sink()`
3. 删除索引节点`delete_at()`
   - `sink()`
4. 获得最值: `peek()`

5. 堆是否为空: `is_empty()`

6. 堆节点个数 `size()`

## 关键问题——上浮下沉

> <http://math.oxford.emory.edu/site/cs171/sinkAndSwi>

### Sinking and Swimming in a Heap

If we are to use a heap as a priority queue, then we need mechanisms for enqueueing (inserting) and dequeueing (removing) items from the collection. Let us consider the case of dequeueing first.

To keep things simple, let us assume the priority queue in question uses a max heap. As such, every parent must be greater than or equal to its children.

Consequently, the item with highest priority will thus be at index 1

in the array. We have to keep the heap in the form of a complete binary tree, so upon removal of this element we'll have to replace it with something.

Of course, filling the hole made by our removal of the maximum element by moving another item in the heap to its location only serves to create a hole elsewhere -- except in one case. If we move the last element in the array (the right-most leaf in the deepest level of the tree), the tree is again made complete.

The only down side of doing this is that **this item might be smaller than one or both of its children**, in which case the heap order of the tree has been violated. Fortunately, we can restore the heap order through a process known as sinking the item. The short description of sinking is this: as long as the item has a greater child, we exchange it with its greatest child.

当我们要加入一个元素的时候，我们需要维持堆的特性，因此在每次把元素插入到数列最后之后，对该索引进行一次上浮操作(swim)通过不断比较父节点，如果是最大堆，子节点大于父节点交换，递归继续向上，如果是最小堆，子节点小于父节点则递归向上

当要弹出最顶端的元素的时候，我们可以复制顶部元素后把原来顶部的空位以最后一个元素代替，然后对顶端做下沉操作(sink)，在孩子存在的情况下，寻找
