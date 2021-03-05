package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
)

const MEMORY_SIZE = 1024 * 8

func run(code *[]byte) *[]int8 {
	memory := make([]int8, MEMORY_SIZE)
	ptr := 0
	jumps := []int{}
	skips := []int{}
	read := readStdin()
	index := 0
	for {
		command := (*code)[index]
		if command == 0 {
			break
		} else if command == '[' {
			if memory[ptr] != 0 {
				jumps = append(jumps, index)
			} else {
				skips = append(skips, index)
			}
		} else if command == ']' {
			if len(skips) == 0 {
				index = jumps[len(jumps)-1]
				jumps = jumps[:len(jumps)-1]
				continue
			} else {
				skips = skips[:len(skips)-1]
			}
		} else if len(skips) > 0 {
			// do nothing
		} else if command == '>' {
			if ptr < MEMORY_SIZE-1 {
				ptr++
			} else {
				ptr = 0
			}
		} else if command == '<' {
			if ptr > 0 {
				ptr--
			} else {
				ptr = MEMORY_SIZE - 1
			}
		} else if command == '+' {
			memory[ptr]++
		} else if command == '-' {
			memory[ptr]--
		} else if command == '.' {
			writeStdout(memory[ptr])
		} else if command == ',' {
			memory[ptr] = read()
		}
		index++
	}
	return &memory
}

func writeStdout(i int8) {
	if i >= 0 {
		fmt.Print(string(rune(i)))
	}
}

func readStdin() func() int8 {
	buff := []byte{}
	return func() int8 {
		if len(buff) == 0 {
			stdin := bufio.NewScanner(os.Stdin)
			for stdin.Scan() {
				buff = append(buff, stdin.Bytes()...)
			}
		}
		if len(buff) == 0 {
			return -1
		}
		b := buff[0]
		buff = buff[1:]
		return int8(b)
	}
}

func readSource(filepath string) *[]byte {
	fp, err := os.Open(filepath)
	if err != nil {
		panic(err)
	}
	defer fp.Close()
	stat, err := fp.Stat()
	if err != nil {
		panic(err)
	}
	buf := make([]byte, stat.Size()+1)
	for {
		n, err := fp.Read(buf)
		if n == 0 {
			break
		}
		if err != nil {
			panic(err)
		}
	}
	return &buf
}

func main() {
	flag.Parse()
	args := flag.Args()
	source := readSource(args[0])
	run(source)
}
