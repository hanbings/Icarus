package main

import "encoding/json"

type CircularBuffer[T any] struct {
	data  []T
	head  int
	tail  int
	count int
	size  int
}

func NewCircularBuffer[T any](size int) *CircularBuffer[T] {
	return &CircularBuffer[T]{
		data:  make([]T, size),
		size:  size,
		count: 0,
		head:  0,
		tail:  0,
	}
}

func (cb *CircularBuffer[T]) MarshalJSON() ([]byte, error) {
	return json.Marshal(cb.GetAll())
}

func (cb *CircularBuffer[T]) Add(item T) {
	if cb.count == cb.size {
		// 如果缓冲区已满，移动头指针
		cb.head = (cb.head + 1) % cb.size
	} else {
		cb.count++
	}
	cb.data[cb.tail] = item
	cb.tail = (cb.tail + 1) % cb.size
}

func (cb *CircularBuffer[T]) GetAll() []T {
	result := make([]T, 0, cb.count)
	for i := 0; i < cb.count; i++ {
		index := (cb.head + i) % cb.size
		result = append(result, cb.data[index])
	}
	return result
}

func (cb *CircularBuffer[T]) Count() int {
	return cb.count
}
