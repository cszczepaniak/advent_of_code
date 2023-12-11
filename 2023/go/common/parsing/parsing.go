package parsing

import "strconv"

func DiscardSpaces(str string) string {
	for len(str) > 0 && str[0] == ' ' {
		str = str[1:]
	}
	return str
}

func DiscardLine(str string) string {
	for len(str) > 0 && str[0] != '\n' {
		str = str[1:]
	}
	if len(str) > 0 {
		return str[1:]
	}
	return str
}

func ParseNumber(str string) (int, string, error) {
	l := 0
	for l < len(str) && str[l] != ' ' && str[l] != '\n' {
		l++
	}

	i, err := strconv.Atoi(str[:l])
	if err != nil {
		return 0, ``, err
	}

	return i, str[l:], nil
}

func ParseSpaceSeparatedNumbers(str string) ([]int, string, error) {
	str = DiscardSpaces(str)

	var n int
	var err error
	var nums []int

	for len(str) > 0 && str[0] != '\n' {
		n, str, err = ParseNumber(str)
		if err != nil {
			return nil, ``, err
		}
		str = DiscardSpaces(str)

		nums = append(nums, n)
	}

	return nums, str, nil
}
