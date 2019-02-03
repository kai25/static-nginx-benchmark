package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"runtime"
)

func main() {
	dat, errFile := ioutil.ReadFile("/data/med.txt")
	if errFile != nil {
		fmt.Println("Can not read file. Exiting")
		return
	}

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.Write(dat)
	})

	runtime.GOMAXPROCS(1)
	fmt.Println("Runnninga server GOMAXPROCS=%v", runtime.GOMAXPROCS(1))

	err := http.ListenAndServe(":80", nil)
	if err != nil {
		fmt.Println("Failed", err)
	}
}
