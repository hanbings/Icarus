package main

import (
	"encoding/json"
	"fmt"
	"os"
)

func LoadConfig() Config {
	env := os.Getenv("CERES_STATUS_CONFIG")
	if env == "" {
		fmt.Println("Environment variable CERES_STATUS_CONFIG is not set")
		os.Exit(1)
	}

	var config Config
	err := json.Unmarshal([]byte(env), &config)
	if err != nil {
		fmt.Println("Parse fail, Please set Environment variable CERES_STATUS_CONFIG as json", err)
		os.Exit(1)
	}

	return config
}
