package main

import (
    "fmt"
    "shopping"
)

func main() {
    a, b := shopping.PriceCheck(78)
    fmt.Printf("price = %f, %t\n", a, b)
}
