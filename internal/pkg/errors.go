package errors

import "fmt"

type InitApp struct {
	Message string
}

func (init *InitApp) Error() string {
	return fmt.Sprintf("Ошибка запуска приложения, %s", init.Message)
}
