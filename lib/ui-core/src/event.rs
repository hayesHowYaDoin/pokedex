#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    NewCharacter(char),
    DeleteCharacter,
    Quit,
    Select,
    Up,
    Down,
    Noop,
}
