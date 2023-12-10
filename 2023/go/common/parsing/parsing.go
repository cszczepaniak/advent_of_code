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
