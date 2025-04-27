//? Workspace Use
pub enum ErrorType {
    UnknownError(Option<String>),
    Unauthorized(Option<String>),
    UserNotFound(Option<String>),
    DuplicatesFound(Option<String>),
    DeviceNotFound(Option<String>),
    ControllableNotFound(Option<String>),
    Unused(Option<String>),
}