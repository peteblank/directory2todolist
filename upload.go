// main.go
package main

import (
	"fmt"
	"io/ioutil"
	"math/rand"
	"net/http"
	"os/exec"
	"strconv"
	"strings"
)

func uploadFile(w http.ResponseWriter, r *http.Request) {
	fmt.Println("File Upload Endpoint Hit")

	// Parse our multipart form, 10 << 20 specifies a maximum
	// upload of 10 MB files.
	r.ParseMultipartForm(10 << 20)
	// FormFile returns the first file for the given key `myFile`
	// it also returns the FileHeader so we can get the Filename,
	// the Header and the size of the file
	file, handler, err := r.FormFile("myFile")
	if err != nil {
		fmt.Println("Error Retrieving the File")
		fmt.Println(err)
		return
	}
	defer file.Close()
	fmt.Printf("Uploaded File: %+v\n", handler.Filename)
	fmt.Printf("File Size: %+v\n", handler.Size)
	fmt.Printf("MIME Header: %+v\n", handler.Header)

	// Create a temporary file within our temp-images directory that follows
	// a particular naming pattern

	tempFile, err := ioutil.TempFile("temp-images", "upload-*.zip")
	if err != nil {
		fmt.Println(err)
	}
	defer tempFile.Close()

	// read all of the contents of our uploaded file into a
	// byte array
	fileBytes, err := ioutil.ReadAll(file)
	if err != nil {
		fmt.Println(err)
	}
	// write this byte array to our temporary file
	tempFile.Write(fileBytes)
	// return that we have successfully uploaded our file!
	fmt.Fprintf(w, "Successfully Uploaded File\n")
	value := strings.Split(tempFile.Name(), "temp-images\\")
	fmt.Printf("   " + value[0] + "   ")
	cmd := exec.Command("/usr/bin/unzip", value[0])
	cmd.Run()
	random := "output" + strconv.FormatInt(int64(rand.Intn(10000)), 10) + ".html"
	cmd2 := exec.Command("/usr/bin/tree", "-H", handler.Filename, "-o", random)
	output, _ := cmd2.CombinedOutput()
	fmt.Printf(string(output))
	err2 := cmd2.Run()
	if err2 != nil {
		fmt.Printf("cmd.Run: %s failed:\n", err2)
	}
	/*output, _ := cmd.CombinedOutput()
	fmt.Println(string(output))*/
	cmd3 := exec.Command("sed", "-i", "/a><br>/a <input type=checkbox></input>", random)
	output2, _ := cmd3.CombinedOutput()
	fmt.Printf(string(output2))
	err3 := cmd3.Run()
	if err3 != nil {
		fmt.Printf("cmd.Run: %s failed:\n", err3)
	}
	cmd4 := exec.Command("sed", "-i", "/green;  }/a input[type=checkbox]:not(:checked) +a{background:white;}input[type=checkbox]:checked + a{border:2px solid green;font-size:24px;background:yellow;}",
		random)
	output3, _ := cmd3.CombinedOutput()
	fmt.Printf(string(output3))
	err4 := cmd4.Run()
	if err4 != nil {
		fmt.Printf("cmd.Run: %s failed:\n", err4)
	}
	cmd5 := exec.Command("sed", "-i", "/<ul>/a <li><a href="+random+">link</a></li>", "index.html")
	output4, _ := cmd3.CombinedOutput()
	fmt.Printf(string(output4))
	err5 := cmd5.Run()
	if err5 != nil {
		fmt.Printf("cmd.Run: %s failed:\n", err5)
	}
}

func setupRoutes() {
	http.HandleFunc("/upload", uploadFile)
	http.ListenAndServe(":8080", nil)
}

// The data struct for the decoded data
// Notice that all fields must be exportable!

// Let's first read the `config.json` file
func main() {
	setupRoutes()
}
