package main

import (
    "fmt"
    "os"
)

const MAXCAP int = 10

type Stack struct {
    arr [MAXCAP]int
    top int
}

func (s *Stack) push(val int) {
    if s.top >= MAXCAP {
        //overflow
        fmt.Println("Error: Overflow")
        os.Exit(1)
    }
    s.arr[s.top] = val
    s.top = s.top+1
}

func (s *Stack) pop() int {
    s.top = s.top-1
    if s.top < 0 {
        //underflow
        fmt.Println("Error: Underflow")
        os.Exit(1)
    }
    return s.arr[s.top]
}

func main(){
    var i int = 0
    s := new(Stack)
    for i=0; i < MAXCAP; i++{
        s.push(i)
        //fmt.Printf("top = %d\n", s.top)
    }

    for i=0; i < MAXCAP; i++{
        fmt.Printf("Pop = %d\n", s.pop())
    }
}
