package queues

const (
	defaultQueue = "default"
	mailingQueue = "mailing"
)

type ItemQueue struct {
	Method  string
	Payload interface{}
}
