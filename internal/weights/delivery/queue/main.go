package queue

func calculateWeightEvent(job Job) error {
	usecase.CalculateWeightEvent{
		repo: repo.NewCalculateWeightEvent(job.db)
		entity: interface{}{}
	}.Call()
}
