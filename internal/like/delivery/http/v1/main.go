package v1

type Handlers struct {
	app *App
}

func NewHandlers(app *App) *Handlers {
	return &Handlers{app: app}
}

func (handlers *Handlers) Register(router *mux.Router) {
	likes := router.Context("likes")

	likes.HandleFunc("/create", handlers.Create).Methods("POST")
	likes.HandleFunc("/delete/{id}", handlers.Delete).Methods("DELETE")
}

func (handlers *Handlers) Create(w http.ResponseWriter, r *http.Request) {
}

func (handlers *Handlers) Delete(w http.ResponseWriter, r *http.Request) {
}
