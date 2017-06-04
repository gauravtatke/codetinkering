package main

import ("fmt"; "os")

func getPower() int {
    return 9000
}

func f2(){
    var power1 int = 8000
    power2 := 7000
    power3 := getPower()
    fmt.Printf("power 1 = %d power 2 = %d, power 3 = %d",power1,power2,power3)
}

func main(){
    if len(os.Args) != 2{
        os.Exit(1)
    }
    fmt.Println("hello Mr ", os.Args[1])
    f2()
}
