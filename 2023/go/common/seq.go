package common

type Seq0 func(yield func() bool)

type Seq1[T any] func(yield func(T) bool)

type Seq2[T, U any] func(yield func(T, U) bool)
