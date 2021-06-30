package main

import (
    "fmt"
)

type Ordered interface {
    type int, float64
}

func min[T Ordered](x, y T) T {
    if x < y {
        return x
    }
    return y
}

// func Map[t1, t2 any](l []t1, f func(t1) t2) []t2 {
//     dst := make([]t2, len(l), len(l))
//     for i, n := range l {
//         dst[i] = f(n)
//     }
//     return dst
// }

func main() {
    a, b := 2, 3
    c, d := 2.5, 3.8
    fmt.Println(min(a, b))
    fmt.Println(min(c, d))

    // intList := []int{0, 1, 2, 3, 4, 5}
    // strList := []string{"generics", "in", "golang"}

    // intListMap := Map(intList, func(i int) int {return i + 1 })
    // strListMap := Map(strList, func(s string) string {return s + "foo"})

    // fmt.Printf("int: %v, type: %T\n", intListMap, intListMap)
    // fmt.Printf("string: %v, type: %T\n", strListMap, strListMap)
}
