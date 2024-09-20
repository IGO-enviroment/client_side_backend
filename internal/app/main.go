package app

import (
	"museum/configs"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"github.com/gorilla/mux"
	"github.com/jmoiron/sqlx"
	"go.uber.org/zap"
)

type App struct {
	db            *sqlx.DB
	log           *zap.Logger
	config        configs.Config
	producerQueue *ProducerQueue
}

func NewApp() *App {
	return &App{}
}

func (app *App) Setup(config configs.Config) error {
	var err error

	app.config = config

	app.db, err = sqlx.Connect("postgres", config.Database)
	if err != nil {
		return err
	}

	cfg := zap.NewProductionConfig()
	cfg.OutputPaths = []string{
		"/var/log/myproject/myproject.log",
	}

	app.log, err = cfg.Build()
	if err != nil {
		return err
	}

	app.producerQueue = nil

	return nil
}

func (app *App) RunWeb(router *mux.Router) {
	done := make(chan os.Signal, 1)
	signal.Notify(done, os.Interrupt, syscall.SIGINT, syscall.SIGTERM)

	server := http.Server{
		Handler:      router,
		Addr:         app.config.HTTPServer.Domain,
		WriteTimeout: app.config.HTTPServer.WriteTimeout,
		ReadTimeout:  app.config.HTTPServer.ReadTimeout,
		IdleTimeout:  app.config.HTTPServer.IdleTimeout,
	}

	go func() {
		if err := server.ListenAndServe(); err != nil {
			app.log.Log(zap.FatalLevel, "Сервер не запущен")
		}
	}()

	<-done
}

func (app *App) RunQueues() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	consumer := NewConsumer(app.db, app.log, app.config)
	consumer.RegisterHandler("send_email", func(job Job) error {
		// Send email logic
		return nil
	})

	go consumer.Run(ctx)
}
