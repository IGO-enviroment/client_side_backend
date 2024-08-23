pub struct AppState {
    db: None,
    client_elastic: None,
    redis: None,
}

pub impl Application {
    // TODO: implement
    pub fn setup(&self) -> Result<None, str> {}

    // TODO: implement
    pub fn state(&self) -> Result<AppState, str> {}

    pub fn run(&self) {
    }
}
