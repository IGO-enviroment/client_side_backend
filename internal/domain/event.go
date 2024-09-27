package domain

type Event struct {
	ID      int  `json:"id"`
	Publish bool `json:"publish"`

	Title       string `json:"title"`
	Description string `json:"description"`

	PreviewURL string `json:"previewURL"`

	TicketCount int `json:"ticketCount"`

	ViewCount int `json:"viewCount"`

	CreatedAt string `json:"createdAt"`
	UpdatedAt string `json:"updatedAt"`
}
