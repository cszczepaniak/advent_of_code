package main

import (
	"flag"
	"fmt"
	"os"
	"path"
	"text/template"
	"time"
)

func main() {
	var (
		year, day int
	)

	flag.IntVar(&year, `year`, -1, `The year to generate for. Default is the current year according to time.Now().`)
	flag.IntVar(&day, `day`, -1, `The day to generate for. Required.`)

	flag.Parse()

	if year == -1 {
		year = time.Now().Year()
	}
	if day == -1 {
		panic(`day must be provided`)
	}

	dayTmpl, err := template.New(`day`).Parse(dayTmplStr)
	if err != nil {
		panic(err)
	}

	dayDir := fmt.Sprintf(`day%d`, day)
	err = os.Mkdir(dayDir, os.ModePerm)
	if err != nil {
		panic(err)
	}

	f, err := os.OpenFile(path.Join(dayDir, `main.go`), os.O_CREATE|os.O_WRONLY, 0o755)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	err = dayTmpl.Execute(f, struct {
		Year, Day int
	}{
		Year: year,
		Day:  day,
	})
	if err != nil {
		panic(err)
	}
}

const dayTmplStr = `package main

import (
	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(
		{{ .Year }}, {{ .Day }},
		part1, part2,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

func part1(input string) int {
	return 0
}

func part2(input string) int {
	return 0
}
`
