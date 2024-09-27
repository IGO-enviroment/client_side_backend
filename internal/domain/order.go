package domain

type Order struct {
	ID          int    `json:"id"`
	UserID      int    `json:"userID"`
	ProductID   int    `json:"productID"`
	ProductName string `json:"productName"`

	Price float64 `json:"price"`

	Status string `json:"status"`

	CreatedAt string `json:"createdAt"`
	UpdatedAt string `json:"updatedAt"`
}
