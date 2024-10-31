package main

type ServiceResponseStatus string

const (
	Success ServiceResponseStatus = "success"
	Failure ServiceResponseStatus = "failure"
	Timeout ServiceResponseStatus = "timeout"
)

type ServiceResponse struct {
	Code   int                   `json:"code"`
	Time   int64                 `json:"time"`
	Status ServiceResponseStatus `json:"status"`
}

type Config struct {
	Port  string        `json:"port"`
	Host  string        `json:"host"`
	Entry []ConfigEntry `json:"entry"`
}

type ConfigEntry struct {
	ServiceName      string `json:"service_name"`
	Endpoint         string `json:"endpoint"`
	Interval         int    `json:"interval"`
	Timeout          int    `json:"timeout"`
	OnlineStatusCode int    `json:"online_status_code"`
}
