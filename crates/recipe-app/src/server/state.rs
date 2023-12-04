use recipe_repository::DieselRepository;

#[derive(Debug, Clone)]
pub struct AppState {
    pub repo: DieselRepository,
}

impl AppState {
    pub fn new(repo: DieselRepository) -> Self {
        Self { repo }
    }
}
