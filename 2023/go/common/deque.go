package common

type Deque[T any] struct {
	len  int
	head *dequeNode[T]
	tail *dequeNode[T]
}

func NewDeque[T any](vals ...T) *Deque[T] {
	d := &Deque[T]{}
	for _, val := range vals {
		d.PushRight(val)
	}
	return d
}

func (d *Deque[T]) Iterate() Seq1[T] {
	return func(yield func(T) bool) {
		for n := d.head; n != nil; n = n.next {
			if !yield(n.val) {
				return
			}
		}
	}
}

func (d *Deque[T]) IterateBackwards() Seq1[T] {
	return func(yield func(T) bool) {
		for n := d.tail; n != nil; n = n.prev {
			if !yield(n.val) {
				return
			}
		}
	}
}

func (d *Deque[T]) Collect() []T {
	return CollectSized[T](d.Iterate(), d.len)
}

func (d *Deque[T]) Len() int {
	return d.len
}

func (d *Deque[T]) PushLeft(val T) {
	newNode := &dequeNode[T]{val: val}
	oldHead := d.head

	newNode.next = oldHead
	if oldHead != nil {
		oldHead.prev = newNode
	}
	d.head = newNode
	if d.tail == nil {
		d.tail = newNode
	}
	d.len++
}

func (d *Deque[T]) PeekLeft() T {
	if d.head == nil {
		panic(`PeekLeft: empty Deque`)
	}
	return d.head.val
}

func (d *Deque[T]) PopLeft() T {
	if d.head == nil {
		panic(`PopLeft: empty Deque`)
	}
	popped := d.head
	d.head = popped.next

	if d.head != nil {
		d.head.prev = nil
	}
	if d.tail == popped {
		d.tail = nil
	}
	d.len--
	return popped.val
}

func (d *Deque[T]) PushRight(val T) {
	newNode := &dequeNode[T]{val: val}
	oldTail := d.tail

	newNode.prev = oldTail
	if oldTail != nil {
		oldTail.next = newNode
	}
	d.tail = newNode
	if d.head == nil {
		d.head = newNode
	}
	d.len++
}

func (d *Deque[T]) PeekRight() T {
	if d.tail == nil {
		panic(`PeekLeft: empty Deque`)
	}
	return d.tail.val
}

func (d *Deque[T]) PopRight() T {
	if d.head == nil {
		panic(`PopRight: empty Deque`)
	}
	popped := d.tail
	d.tail = popped.prev
	if d.tail != nil {
		d.tail.next = nil
	}
	if d.head == popped {
		d.head = nil
	}
	d.len--
	return popped.val
}

type dequeNode[T any] struct {
	val  T
	prev *dequeNode[T]
	next *dequeNode[T]
}
