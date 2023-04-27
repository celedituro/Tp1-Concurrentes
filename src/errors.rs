#[derive(Debug, PartialEq)]
pub enum Error {
    NotFileInput,
    FileNotFound,
    WrongFileFormat,
    NotEnoughIngredient,
    NoMoreOrders,
    CantWriteOrdersLock,
    CantHaveContainerLock,
    CantReadOrdersLock,
}
