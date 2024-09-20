package queues

const (
	defaultQueue = "default"
	mailingQueue = "mailing"
)

type ProducerQueue struct {
	log    *zap.Logger
	config configs.Config
	queues []string
}

type ItemQueue struct {
	Method  string
	Payload interface{}
}

func NewProducerQueue(log *zap.Logger, config configs.Config) *ProducerQueue {
	return &ProducerQueue{
		log:    log,
		config: config,
		queues: []string{defaultQueue, mailingQueue},
	}
}

func (q *Queue) Add(db *sqlx.DB, queueName string, item ItemQueue) error {
	if !contains(q.queues, queueName) {
		q.log.Log(zap.ErrorLevel, "Отсутствует такая очередь", zap.String("queue_name", queueName))
		return fmt.Errorf("отсутствует такая очередь: %s", queueName)
	}

	// Add item to the queue
	_, err := db.Exec(`INSERT INTO queue (queue_name, item) VALUES ($1, $2)`, queueName, item)
	if err != nil {
		q.log.Log(zap.ErrorLevel, "Ошибка добавления в очередь", zap.Error(err))
		return err
	}

	q.log.Log(zap.InfoLevel, "Записано в очередь", zap.String("queue_name", queueName))
	return nil
}

type Consumer struct {
	db     *sqlx.DB
	log    *zap.Logger
	config configs.Config
	queues []string
	processItems map[string, func() ([]string, error)]
}

func NewConsumer(db *sqlx.DB, log *zap.Logger, config configs.Config) *Consumer {
	return &Consumer{
		db:     db,
		log:    log,
		config: config,
		queues: []string{defaultQueue, mailingQueue},
	}
}

func (c *Consumer) Run(ctx context.Context) {
	for _, queueName := range c.queues {
		go c.consumeQueue(ctx, queueName)
	}
	<-ctx.Done()
}

func (c *Consumer) consumeQueue(ctx context.Context, queueName string) {
	for {
		select {
		case <-ctx.Done():
			return
		default:
			item, err := c.getItem(ctx, queueName)
			if err != nil {
				c.log.Log(zap.ErrorLevel, "Ошибка получения из очереди", zap.Error(err))
				continue
			}

			err = c.processItem(ctx, queueName, item.Item)
			if err != nil {
				c.log.Log(zap.ErrorLevel, "Ошибка обработки из очереди", zap.Error(err))
				continue
			}

			err = c.markProcessedItem(ctx, Item.ID)
			if err != nil {
				c.log.Log(zap.ErrorLevel, "Ошибка обновления статуса в очереди", zap.Error(err))
			}
		}
	}
}

func (c *Consumer) RegisterHandler(method string, handler func(ctx context.Context, payload interface{}) error) {
    c.processItems[method] = handler
}

type ProcceItem struct {
	ID int
	Item ItemQueue
}

func (c *Consumer) getItem(ctx context.Context, queueName string) (interface{}, error) {
	var item ProcceItem
	err := c.db.QueryRowx(`SELECT id, item FROM queue WHERE processed = false AND queue_name = $1 ORDER BY id LIMIT 1`, queueName).Scan(&item)
	if err == sql.ErrNoRows {
		return nil, nil
	} else if err != nil {
		return nil, err
	}
	return item, nil
}

func (c *Consumer) processItem(ctx context.Context, queueName string, item interface{}) error {
	if _, ok := c.processItems[item.Method]; !ok {
        c.log.Log(zap.ErrorLevel, "Отсутствует обработчик для этого метода", zap.String("method", item.Method))
        return fmt.Errorf("отсутствует обработчик для этого метода: %s", item.Method)
    }

    return c.processItems[item.Method](ctx, item.Payload)
}

func (c *Consumer) markProcessedItem(ctx context.Context, id int) error {
	_, err := c.db.Exec(`UPDATE queue SET processed = true WHERE id = $1`, id)
    return err
}
