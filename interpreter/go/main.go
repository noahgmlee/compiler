package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	args := os.Args
	argCount := len(args) - 1
	if argCount > 1 {
		fmt.Println("Usage: lox [script]")
		os.Exit(64)
	} else if argCount == 1 {
		runFile(args[1])
	} else {
		fmt.Println("Starting Lox Prompt! :)")
		runPrompt()
	}
}

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
