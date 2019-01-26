package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
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

	fmt.Println("Runnning server")
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		fmt.Println("Failed", err)
	}
}
