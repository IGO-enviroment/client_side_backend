package configs

import "time"

type HTTPServerCfg struct {
	Domain       string
	ReadTimeout  time.Duration
	WriteTimeout time.Duration
	IdleTimeout  time.Duration
}

type Config struct {
	Database   string
	HTTPServer HTTPServerCfg
}

func NewConfig() *Config {
	return &Config{
		Database: "user=foo dbname=bar sslmode=disable",
		HTTPServer: HTTPServerCfg{
			Domain:       "127.0.0.1:8000",
			ReadTimeout:  15 * time.Second,
			WriteTimeout: 15 * time.Second,
			IdleTimeout:  2 * time.Minute,
		},
	}
}
