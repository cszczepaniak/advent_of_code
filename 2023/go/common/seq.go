package common

type Seq0 func(yield func() bool)

type Seq1[T any] func(yield func(T) bool)

type Seq2[T, U any] func(yield func(T, U) bool)

func IterLines(str string) Seq1[string] {
	return func(yield func(string) bool) {
		start := 0
		for i := range len(str) {
			if str[i] == '\n' {
				res := yield(str[start:i])
				if !res {
					return
				}
				start = i + 1
			}
		}
		if start < len(str) {
			yield(str[start:])
		}
	}
}
