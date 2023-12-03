package common

func IterLines(str string) Seq1[string] {
	return IterSplitString(str, '\n')
}

func IterSplitString(str string, delim byte) Seq1[string] {
	return func(yield func(string) bool) {
		start := 0
		for i := range len(str) {
			if str[i] == delim {
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
