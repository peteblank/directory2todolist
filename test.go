// main.go
package main

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"reflect"
)

// The data struct for the decoded data
// Notice that all fields must be exportable!
type Users struct {
	Users []Data `json:"data"`
}
type Data struct {
	Type     string `json:"type"`
	Name     string `json:"name"`
	Contents []content
}
type content struct {
	Type     string `json:"type"`
	Name     string `json:"name"`
	Contents []content
}

func data2(reflect.Value) {

}
func main() {
	// Let's first read the `config.json` file

	content, _ := ioutil.ReadFile("output.json")

	var data Users
	json.Unmarshal(content, &data)
	i := -1
	for range data.Users[0].Contents[0].Contents[1].Contents {
		i++
		log.Printf("results: %s\n", data.Users[0].Contents[0].Contents[1].Contents[i])
	}

}
