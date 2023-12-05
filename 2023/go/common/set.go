package common

type Set[T comparable] map[T]struct{}

func NewSet[T comparable](vals ...T) Set[T] {
	s := make(Set[T], len(vals))
	for _, val := range vals {
		s[val] = struct{}{}
	}
	return s
}

func NewSetWithCapacity[T comparable](cap int) Set[T] {
	return make(Set[T], cap)
}

func (s Set[T]) Contains(v T) bool {
	_, ok := s[v]
	return ok
}

func (s Set[T]) Insert(v T) bool {
	if !s.Contains(v) {
		s[v] = struct{}{}
		return true
	}
	return false
}
