#[derive(Debug, Default, PartialEq)]
pub enum UserRole {
    #[default]
    Guest,
    Admin,
    School,
    Player,
    Analyst,
    CollectionPoint,
}

#[derive(Debug, Default)]
pub struct App {
    pub role: UserRole,
    pub exit: bool,
}

