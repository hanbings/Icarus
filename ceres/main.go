package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
)

var Configs = LoadConfig()
var Services = make(map[string]*CircularBuffer[ServiceResponse])

func main() {
	for _, entry := range Configs.Entry {
		buffer := NewCircularBuffer[ServiceResponse](60)
		Services[entry.ServiceName] = buffer

		go StartHttpRequest(entry.ServiceName, entry.Endpoint, entry.OnlineStatusCode, entry.Interval, entry.Timeout)
	}

	router := gin.Default()
	router.GET("/", GetStatus)
	router.GET("/services", GetServices)
	router.GET("/status", GetStatus)
	err := router.Run(Configs.Host + ":" + Configs.Port)
	if err != nil {
		fmt.Println("Start gin fail", err)
		return
	}
}
