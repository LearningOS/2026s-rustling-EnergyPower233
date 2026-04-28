/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 1. 新员工入职，排在公司最末尾
        self.items.push(value);
        self.count += 1; // 公司总人数加 1

        let mut current_idx = self.count;

        // 2. 只要还没爬到 CEO 的位置 (idx > 1)，就尝试挑战直系主管
        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx);

            // 如果新员工 比 主管 更符合老板要求（在最小堆里就是值更小，最大堆就是值更大）
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                // 篡位！互换工位
                self.items.swap(current_idx, parent_idx);
                // 现在新员工坐到了主管的位置，准备继续挑战下一任主管
                current_idx = parent_idx;
            } else {
                // 遇到比自己强的主管了，挑战结束，老实干活
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 如果存在右节点，并且右节点 比 左节点 更符合优先要求
        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        } else {
            // 否则就选左节点（可能只有左节点，或者左节点更强）
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 如果公司破产（没人了），直接返回 None
        if self.count == 0 {
            return None;
        }

        // 1. 把 CEO (下标 1) 和 最底层的实习生 (下标 count) 互换位置
        self.items.swap(1, self.count);

        // 2. 原来的 CEO 现在在数组末尾，我们把他踢出局并记录下来准备返回
        let result = self.items.pop();
        self.count -= 1; // 公司总人数减 1

        // 3. 现在的假 CEO (原实习生) 坐在下标 1 的位置，开始降级流程
        let mut current_idx = 1;

        // 只要他还有直系下属，就面临被下属篡位的风险
        while self.children_present(current_idx) {
            // 找出两个下属里比较强的那个
            let child_idx = self.smallest_child_idx(current_idx);

            // 如果最强的下属 比 假 CEO 还要强
            if (self.comparator)(&self.items[child_idx], &self.items[current_idx]) {
                // 假 CEO 降级，最强下属升职
                self.items.swap(current_idx, child_idx);
                current_idx = child_idx;
            } else {
                // 假 CEO 居然比最强的下属还强，稳住阵脚，降级结束
                break;
            }
        }

        // 4. 返回真正离职的那个老 CEO
        result
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
