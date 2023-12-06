package common

type ByteSet struct {
	bits [4]uint64
}

func NewByteSet() ByteSet {
	return ByteSet{}
}

func (s *ByteSet) Insert(b byte) {
	idx := b / 64
	s.bits[idx] = s.bits[idx] | 1<<uint64((b-idx*64))
}

func (s ByteSet) Contains(b byte) bool {
	idx := b / 64
	return s.bits[idx]&(1<<uint64((b-idx*64))) > 0
}
