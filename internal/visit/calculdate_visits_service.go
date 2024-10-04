package visit

type Visit struct {
	repo Repository
}

func NewVisit(repo Repository) *Visit {
	return &Visit{repo: repo}
}
