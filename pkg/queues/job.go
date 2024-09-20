package queues

type Callback struct {
	afterFailure []func(err error) error
	afterSuccess []func() error
}

type Job struct {
	consumer *Consumer

	payload interface{}

	ctx *context.Context

	item proccedItem

	callbacks []Callback
}

type ProcceItem struct {
	ID   int
	Item ItemQueue
}

func NewJob(consumer *Consumer, payload interface{}, ctx *context.Context) *Job {
	return &Job{
		consumer: consumer,
		payload:  payload,
		ctx:      ctx,
	}
}

func (job *Job) Start( string) {
	jobName := job.item.Item.Method
	action, ok := job.consumer.registredJobs[jobName]
	if !ok {
		job.consumer.log.Log(zap.ErrorLevel, "Отсутствует обработчик для этого метода", jobName)
		return fmt.Errorf("отсутствует обработчик для этого метода: %s", jobName)
	}

	err := action(job)
	if err != nil {
		job.AfterFailure(err)
		return err
	}

	return
}

func (job *Job) GetItem() {
	err := c.db.QueryRowx(
		`UPDATE queue
		SET procced_at = CURRENT_DATE, processed = true
		WHERE processed = false AND queue_name = $1 AND (run_at < $2 OR run_at IS NULL) ORDER BY id LIMIT 1
		RETURNING id, item`,
		queueName,
		time.Now().UTC()
	).Scan(&job.item)

	if err == sql.ErrNoRows {
		return nil, nil
	} else if err != nil {
		return nil, err
	}

	return item, nil
}

func (job *Job) AfterFailure(err error) {
	job.consumer.log.Log(zap.ErrorLevel, "Ошибка обработки из очереди", zap.Error(err))
}

func (job *Job) RegisterCallback(action string, func(job *Job) error) error {
	job.callbacks = append(job.callbacks, Callback{})
}
