package v1

type Handlers struct {
	app *App
}

func NewHandlers(app *App) *Handlers {
	return &Handlers{app: app}
}

func (handlers *Handlers) Register(router *mux.Router) {
	events := router.Context("events")

	events.HandleFunc("/create", handlers.Create).Methods("POST")
	events.HandleFunc("/update/{id}", handlers.Update).Methods("PUT")
	events.HandleFunc("/{id}", handlers.Get).Methods("GET")
	events.HandleFunc("/", handlers.List).Methods("GET")
	events.HandleFunc("/search", handlers.Search).Methods("GET")
	events.HandleFunc("/publish", handlers.Publish).Methods("POST")
	events.HandleFunc("/unpublish", handlers.Unpublish).Methods("POST")
	events.HandleFunc("/billboard", handlers.Billboard).Methods("GET")
}

func (handlers *Handlers) Create(w http.ResponseWriter, r *http.Request) {
}

func (handlers *Handlers) Update(w http.ResponseWriter, r *http.Request) {
}

func (handlers *Handlers) Get(w http.ResponseWriter, r *http.Request) {
}

func (handlers *Handlers) List(w http.ResponseWriter, r *http.Request) {
}

func (handlers *Handlers) Search(w http.ResponseWriter, r *http.Request) {
}

func (handlers *Handlers) Publish(w http.ResponseWriter, r *http.Request) {
}

func (handlers *Handlers) Unpublish(w http.ResponseWriter, r *http.Request) {
}

func (handlers *Handlers) Billboard(w http.ResponseWriter, r *http.Request) {
}
