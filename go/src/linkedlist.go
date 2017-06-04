package main

import (
    "fmt"
    //"os"
)

type lnode struct{
    key int
    next *lnode
}

func insert(head *lnode, val int) *lnode{
    temp := new(lnode)
    temp.key = val
    temp.next = head.next
    head.next = temp
    return head
}

func delete(head *lnode, val int){
    curr := head
    for curr.next != nil{
        if curr.next.key == val{
            curr.next = curr.next.next
            return
            }
        curr = curr.next
    }
}

func printlist(head *lnode){
    curr := head.next
    for curr != nil{
        fmt.Printf("%d->", curr.key)
        curr = curr.next
    }
    fmt.Println(curr)
}

func main(){
    head := new(lnode)
    for i:= 1; i < 10; i++{
        head = insert(head, i)
    }
    printlist(head)
    delete(head, 1)
    printlist(head)
    delete(head, 5)
    printlist(head)
    delete(head, 9)
    printlist(head)
}
