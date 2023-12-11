package common

func Zip[T any, U any, S1 ~[]T, S2 ~[]U](s1 S1, s2 S2) Seq2[T, U] {
	return func(yield func(T, U) bool) {
		for i, t := range s1 {
			if i >= len(s2) {
				return
			}
			if !yield(t, s2[i]) {
				return
			}
		}
	}
}
