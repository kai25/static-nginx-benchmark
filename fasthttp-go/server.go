package main

import (
	"fmt"
	"log"
	"io/ioutil"
	"github.com/valyala/fasthttp"
)

func main() {
	fileBytes, errFile := ioutil.ReadFile("/data/med.txt")
	if errFile != nil {
		fmt.Println("Can not read file. Exiting")
		return
	}

	h := func (ctx *fasthttp.RequestCtx) {
		ctx.SetBody(fileBytes)

		ctx.SetContentType("text/plain; charset=utf8")	
	}

	if err := fasthttp.ListenAndServe("0.0.0.0:8080", h); err != nil {
		log.Fatalf("Error in ListenAndServe: %s", err)
	}
}