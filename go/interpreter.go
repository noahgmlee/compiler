package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func run(input string) {
	lexer := NewLexer(input)
	tokens := lexer.ScanTokens()
	for _, token := range tokens {
		fmt.Println(token.ToString())
	}
}

func runFile(path string) {
	data, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Error reading file:", err)
		fmt.Println("provided path: ", path)
		return
	}
	content := string(data)
	run(content)
}

func runPrompt() {
	scanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print(">> ")
		if !scanner.Scan() {
			break
		}
		input := strings.TrimSpace(scanner.Text())
		run(input)
	}
}
