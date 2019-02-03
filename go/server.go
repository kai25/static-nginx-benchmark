package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"runtime"
)

func loadFile(fileName string) []byte {
	fileBytes, err := ioutil.ReadFile(fileName)
	if err != nil {
		panic("Can not read file. Exiting")
	}
	return fileBytes
}

func main() {
	medFile := loadFile("/data/med.txt")
	smallFile := loadFile("/data/small.txt")
	bigFile := loadFile("/data/big.txt")

	http.HandleFunc("/med.txt", func(w http.ResponseWriter, r *http.Request) {
		w.Write(medFile)
	})

	http.HandleFunc("/small.txt", func(w http.ResponseWriter, r *http.Request) {
		w.Write(smallFile)
	})

	http.HandleFunc("/big.txt", func(w http.ResponseWriter, r *http.Request) {
		w.Write(bigFile)
	})

	runtime.GOMAXPROCS(1)
	fmt.Println("Runnninga server GOMAXPROCS=%v", runtime.GOMAXPROCS(1))

	err := http.ListenAndServe(":80", nil)
	if err != nil {
		fmt.Println("Failed", err)
	}
}
