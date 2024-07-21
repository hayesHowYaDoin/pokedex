#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    NewCharacter(char),
    DeleteCharacter,
    Up,
    Down,
    Noop,
}