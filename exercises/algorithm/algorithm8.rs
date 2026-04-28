/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>, // q2 作为一个中转站
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        // 1. 新来的元素，先进入空的中转站 q2
        self.q2.enqueue(elem);

        // 2. 把 q1 里的老元素全部出队，并跟在新元素后面进入 q2
        // 这样就实现了“后来居上”的效果
        while !self.q1.is_empty() {
            if let Ok(val) = self.q1.dequeue() {
                self.q2.enqueue(val);
            }
        }

        // 3. 此时 q2 里面的顺序就是完美的栈顺序了。
        // 我们用 Rust 标准库里的“魔法”，直接把 q1 和 q2 互换。
        // 这样 q1 依然是我们存放最终结果的主力，q2 又变回了空的中转站。
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        // 因为我们在 push 的时候已经排好了顺序，最上面的元素就在 q1 的队首。
        // 直接让 q1 出队即可。
        match self.q1.dequeue() {
            Ok(val) => Ok(val),
            // 题目测试用例要求报错信息是 "Stack is empty"，所以要转换一下错误信息
            Err(_) => Err("Stack is empty"),
        }
    }

    pub fn is_empty(&self) -> bool {
        // 只要主力队伍 q1 是空的，栈就是空的
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
