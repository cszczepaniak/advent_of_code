package common

type Seq0 func(yield func() bool)

type Seq1[T any] func(yield func(T) bool)

type Seq2[T, U any] func(yield func(T, U) bool)

func Collect[T any](seq Seq1[T]) []T {
	return CollectSized(seq, 0)
}

func CollectSized[T any](seq Seq1[T], sz int) []T {
	res := make([]T, 0, sz)
	for val := range seq {
		res = append(res, val)
	}
	return res
}
