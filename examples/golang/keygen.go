package main

/*
#cgo LDFLAGS: -L./lib -lctss
#include "./lib/ctss.h"
*/
import "C"
import (
	"fmt"
	"log"
	"os"
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
	data := C.keygen(C.CString("http://localhost:8000/"), C.CString("default-keygen"), C.uint16_t(index), 1, 3)

	file, err := os.Create(fmt.Sprintf("local-share%d.json", index))
	if err != nil {
		log.Fatalln(err)
	}
	defer file.Close()

	_, err = file.WriteString(C.GoString(data))
	if err != nil {
		log.Fatalln(err)
	}
}
