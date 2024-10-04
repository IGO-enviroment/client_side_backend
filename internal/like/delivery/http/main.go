package v1

type Handlers struct {
	app *App
}

func NewHandlers(app *App) *Handlers {
	return &Handlers{app: app}
}

func (handlers *Handlers) Register(router *mux.Router) {
	likes := router.Context("likes")

	likes.HandleFunc("/update", handlers.Create).Methods("PATCH")
}

func (handlers *Handlers) Update(w http.ResponseWriter, r *http.Request) {
}
