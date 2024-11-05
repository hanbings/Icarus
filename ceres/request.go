package main

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net/http"
	"time"
)

func StartHttpRequest(serviceName string, endpoint string, expectStatusCode int, interval int, timeout int) {
	client := &http.Client{}
	for {
		time.Sleep(time.Duration(interval) * time.Minute)

		ctx, cancel := context.WithTimeout(context.Background(), time.Duration(timeout)*time.Second)
		req, err := http.NewRequestWithContext(ctx, http.MethodGet, endpoint, nil)
		services := Services[serviceName]
		if err != nil {
			fmt.Println("Request new fail:", err)
			cancel()
			services.Add(ServiceResponse{Code: 0, Time: time.Now().Unix(), Status: Failure})

			continue
		}
		response, err := client.Do(req)
		if err != nil {
			if errors.Is(ctx.Err(), context.DeadlineExceeded) {
				services.Add(ServiceResponse{Code: 0, Time: time.Now().Unix(), Status: Timeout})
			} else {
				services.Add(ServiceResponse{Code: 0, Time: time.Now().Unix(), Status: Failure})
			}
			cancel()
			continue
		}

		func(Body io.ReadCloser) {
			err := Body.Close()
			if err != nil {
				services.Add(ServiceResponse{Code: 0, Time: time.Now().Unix(), Status: Failure})
				fmt.Println("Close fail:", err)
			}
		}(response.Body)

		fmt.Println("Service:", serviceName, "Endpoint:", endpoint, "StatusCode:", response.StatusCode, "Time:", time.Now().Unix())

		if response.StatusCode == expectStatusCode {
			services.Add(ServiceResponse{Code: response.StatusCode, Time: time.Now().Unix(), Status: Success})
		} else {
			services.Add(ServiceResponse{Code: response.StatusCode, Time: time.Now().Unix(), Status: Failure})
		}

		cancel()
	}
}
