package event

type Usecase interface {
	CalculateWeightEvent(interface{}) error
	UpdateEvent(interface{}) error
	PublishEvent(interface{}) error
	UnpublishEvent(interface{}) error
	SearchEvents(interface{}) ([]interface{}, error)
	BillboardEvents(interface{}) ([]interface{}, error)
}

type usecase struct {
	repo Repository
}

func NewUsecase(repo Repository) Usecase {
	return &usecase{repo: repo}
}
