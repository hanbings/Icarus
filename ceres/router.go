package main

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"time"
)

func GetServices(c *gin.Context) {
	c.JSON(http.StatusOK, Services)
}

func GetStatus(c *gin.Context) {
	c.IndentedJSON(http.StatusOK, struct {
		Code        int    `json:"code"`
		Message     string `json:"message"`
		Time        int64  `json:"time"`
		Version     string `json:"version"`
		Application string `json:"application"`
		Endpoint    string `json:"endpoint"`
	}{
		Code:        200,
		Message:     "Success ╭(○｀∀´○)╯",
		Time:        time.Now().Unix(),
		Version:     "v1.0.0",
		Application: "Ceres",
		Endpoint:    "/services",
	})
}
