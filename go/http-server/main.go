package main

import (
	"bufio"
	"bytes"
	"compress/gzip"
	"fmt"
	"net"
	"os"
	"path/filepath"
	"strings"
)

type Request struct {
	method  string
	target  string
	version string
	headers map[string]string
	body    []byte
}

func main() {
	protocol := "tcp"
	tcpAddr, err := net.ResolveTCPAddr(protocol, "127.0.0.1:8080")
	if err != nil {
		fmt.Println(err)
		return
	}
	listener, err := net.ListenTCP(protocol, tcpAddr)
	if err != nil {
		fmt.Println(err)
		return
	}

	defer listener.Close()

	for {
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println(err)
			continue
		}
		fmt.Println("accept: ", conn.RemoteAddr().String())
		go handleConnection(conn)
	}
}

func handleConnection(conn net.Conn) {
	defer conn.Close()

	reader := bufio.NewReader(conn)
	writer := bufio.NewWriter(conn)
	for {
		messageBuf := make([]byte, 1024*8)
		messageLen, err := reader.Read(messageBuf)
		if err != nil {
			return
		}
		message := string(messageBuf[:messageLen])
		slice := strings.Split(message, "\r\n")
		req := Request{headers: map[string]string{}}
		for index, line := range slice {
			if len(line) == 0 {
				break
			}
			if index == 0 {
				splits := strings.Split(line, " ")
				req.method = splits[0]
				req.target = splits[1]
				req.version = splits[2]
			} else {
				splits := strings.Split(line, ":")
				req.headers[strings.Trim(splits[0], " ")] = strings.Trim(splits[1], " ")
			}
			// TODO: body
		}
		fmt.Println(req)

		filePath := filepath.Join("/home/omura/work/repositories/github.com/omurakazuaki/brainfuck-interpreter/build", req.target)
		stat, err := os.Stat(filePath)
		if err != nil {
			// TODO: 404
			res := "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nContent-Type: text/plan\r\n\r\n"
			writer.Write([]byte(res))
			writer.Flush()
			continue
		}
		if stat.IsDir() {
			filePath = filepath.Join(filePath, "index.html")
			stat, err = os.Stat(filePath)
			if err != nil {
				// TODO: 404
				res := "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nContent-Type: text/plan\r\n\r\n"
				writer.Write([]byte(res))
				writer.Flush()
				continue
			}
		}

		writer.Write([]byte("HTTP/1.1 200 OK\r\n"))
		writer.Write([]byte(fmt.Sprintf("Content-Type: %s\r\n", resolveContentType(filepath.Ext(filePath)))))
		if 16384 < stat.Size() {
			writer.Write([]byte(fmt.Sprintf("Content-Encoding: %s\r\n", "gzip")))
			writer.Write([]byte(fmt.Sprintf("Transfer-Encoding: %s\r\n", "chunked")))
			writer.Write([]byte("\r\n"))
			var gzipBuff bytes.Buffer
			gzipWriter := gzip.NewWriter(&gzipBuff)
			defer gzipWriter.Close()
			gzipWriter.Write(readFile(filePath))
			gzipWriter.Flush()
			for {
				buf := make([]byte, 16384)
				n, err := gzipBuff.Read(buf)
				if n == 0 {
					writer.Write([]byte("0\r\n\r\n"))
					writer.Flush()
					break
				}
				if err != nil {
					if err.Error() == "EOF" {
						writer.Write([]byte("0\r\n\r\n"))
						writer.Flush()
						break
					}
					panic(err)
				}
				writer.Write([]byte(fmt.Sprintf("%x\r\n", n)))
				writer.Write(buf[0:n])
				writer.Write([]byte("\r\n"))
				writer.Flush()
			}
		} else {
			writer.Write([]byte(fmt.Sprintf("Content-Length: %d\r\n", stat.Size())))
			writer.Write([]byte("\r\n"))
			writer.Write(readFile(filePath))
			writer.Flush()
		}
	}
}

var types = map[string][]string{
	"text/html":              {"html", "htm", "shtml"},
	"text/css":               {"css"},
	"image/gif":              {"gif"},
	"image/png":              {"png"},
	"image/jpeg":             {"jpeg", "jpg"},
	"image/svg+xml":          {"svg", "svgz"},
	"image/x-icon":           {"ico"},
	"application/javascript": {"js"},
	"application/json":       {"json", "map"},
}

func resolveContentType(ext string) string {
	ext = strings.TrimPrefix(ext, ".")
	for t, exts := range types {
		for _, e := range exts {
			if e == ext {
				return t
			}
		}
	}
	return "text/html"
}

func readFile(filepath string) []byte {
	fp, err := os.Open(filepath)
	if err != nil {
		panic(err)
	}
	defer fp.Close()
	stat, err := fp.Stat()
	if err != nil {
		panic(err)
	}
	buf := make([]byte, stat.Size())
	_, err = fp.Read(buf)
	if err != nil {
		panic(err)
	}
	return buf
}
