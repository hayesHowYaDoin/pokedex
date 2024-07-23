#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    NewCharacter(char),
    DeleteCharacter,
    Select,
    Up,
    Down,
    Noop,
}