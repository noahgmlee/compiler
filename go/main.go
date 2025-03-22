package main

import (
	"fmt"
	"os"
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
