package main

/*
#cgo LDFLAGS: -L./lib -lctss
#include "./lib/ctss.h"
*/
import "C"
import (
	"fmt"
	"log"
	"strconv"
)

func main() {
	fmt.Println("input index:")
	var indexStr string
	_, err := fmt.Scanln(&indexStr)
	if err != nil {
		log.Fatalln(err)
	}

	index, err := strconv.Atoi(indexStr)
	if err != nil {
		log.Fatalln(err)
	}
	signature := C.sign(
		C.CString("http://localhost:8000/"),
		C.CString("default-keygen"),
		C.CString(fmt.Sprintf("local-share%d.json", index)),
		C.CString("1,2"),
		C.CString("hello"),
	)

	fmt.Println(C.GoString(signature))
}
