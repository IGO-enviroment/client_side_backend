package like

type UpdateService struct {
	repo Repository
	app  *Application
}

func NewUpdateService(repo Repository, app *Application) *UpdateService {
	return &UpdateService{repo: repo, app: app}
}

func (u *UpdateService) Call(input UpdateInput) error {
	like := &Like{
		UserID:    input.UserID,
		PostID:    input.PostID,
		Timestamp: time.Now(),
	}

	like, err := u.findLike(like.UserID, like.EventID)
	if err != nil && err != sql.ErrNoRows {
		u.app.Logger.Error("Error finding like", zap.Error(err))
		return err
	}

	if like.ID != 0 {
		err := u.repo.Delete(input.UserID, input.EventID)
		if err != nil {
			u.app.Logger.Error("Error deleting like", zap.Error(err))
			return err
		}

		return nil
	} else {
		err := u.repo.Save(like)
		if err != nil {
			u.app.Logger.Error("Error saving like", zap.Error(err))
			return err
		}

		return nil
	}
}

func (u *UpdateService) findLike(userID, eventID int) (*Like, error) {
	like, err := u.repo.FindBy(u.repo.Options{userID: userID, eventID: eventID})
	if err != nil {
		return nil, err
	}

	return &like, nil
}
