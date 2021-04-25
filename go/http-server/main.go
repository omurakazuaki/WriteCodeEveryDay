package main

import (
	"fmt"
	"net"
	"time"
)

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

	conn.SetReadDeadline(time.Now().Add(10 * time.Second))

	messageBuf := make([]byte, 1024)
	messageLen, err := conn.Read(messageBuf)
	if err != nil {
		fmt.Println(err)
		return
	}
	message := string(messageBuf[:messageLen])
	fmt.Println(message)

	res := "HTTP/1.1 200 OK\r\nContent-Length: 13\r\nContent-Type: text/plan\r\n\r\nHello, World!"
	conn.Write([]byte(res))
}
