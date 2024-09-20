package queues

type ProducerQueue struct {
	log    *zap.Logger
	config configs.Config
	queues []string
}

func NewProducerQueue(log *zap.Logger, config configs.Config) *ProducerQueue {
	return &ProducerQueue{
		log:    log,
		config: config,
		queues: []string{defaultQueue, mailingQueue},
	}
}

func (q *Queue) Add(db *sqlx.DB, queueName string, item ItemQueue, run_at *time.Time) error {
	if !contains(q.queues, queueName) {
		q.log.Log(zap.ErrorLevel, "Отсутствует такая очередь", zap.String("queue_name", queueName))
		return fmt.Errorf("отсутствует такая очередь: %s", queueName)
	}

	if run_at == nil {
		run_at = time.Now().UTC()
	}

	// Add item to the queue
	_, err := db.Exec(`INSERT INTO queue (queue_name, item, run_at) VALUES ($1, $2)`, queueName, item, run_at)
	if err != nil {
		q.log.Log(zap.ErrorLevel, "Ошибка добавления в очередь", zap.Error(err))
		return err
	}

	q.log.Log(zap.InfoLevel, "Записано в очередь", zap.String("queue_name", queueName))
	return nil
}
