package queues


type ProcceItem struct {
	ID   int
	Item ItemQueue
}

type Consumer struct {
	db     *sqlx.DB
	log    *zap.Logger
	config configs.Config
	queues []string
	registredJobs map[string, func() ([]string, error)]
}

func NewConsumer(db *sqlx.DB, log *zap.Logger, config configs.Config) *Consumer {
	return &Consumer{
		db:     db,
		log:    log,
		config: config,
		queues: []string{defaultQueue, mailingQueue},
	}
}

func (c *Consumer) RegisterJob(method string, handler func(job *Job) error) {
    c.registredJobs[method] = handler
}

func (c *Consumer) Run(ctx context.Context) {
	for _, queueName := range c.queues {
		go c.workersQueue(ctx, queueName)
	}
	<-ctx.Done()
}

func (c *Consumer) workersQueue(ctx context.Context, queueName) {
	workers := 10
    jobs := make(chan int)
    wg := &sync.WaitGroup{}

	defer wg.Wait()

	for i := 0; i < workers; i++ {
		wg.Add(1)
		go func(wg *sync.WaitGroup) {
			for {
				select {
                case <-ctx.Done():
                    wg.Done()
                    return
                default:
					item, err := c.getItem(ctx, queueName)
					if item == nil {
                        continue
					}

					if err != nil {
						c.log.Log(zap.ErrorLevel, "Ошибка получения из очереди", zap.Error(err))
						continue
					}

					NewJob(c, item.Item.Payload, ctx).Start(item.Item.Method)

					wg.Done()
                }
			}
        }(wg)
	}
}

func (c *Consumer) getItem(ctx context.Context, queueName string) (interface{}, error) {
	var item ProcceItem

	err := c.db.QueryRowx(
		`UPDATE queue
		SET procced_at = CURRENT_DATE, processed = true
		WHERE processed = false AND queue_name = $1 AND (run_at < $2 OR run_at IS NULL) ORDER BY id LIMIT 1
		RETURNING id, item`,
		queueName,
		time.Now().UTC()
	).Scan(&item)

	if err == sql.ErrNoRows {
		return nil, nil
	} else if err != nil {
		return nil, err
	}

	return item, nil
}
