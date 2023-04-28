#[derive(Debug, PartialEq)]
pub enum Error {
    NotFileInput,
    FileNotFound,
    WrongFileFormat,
    NotEnoughIngredient,
    NoMoreOrders,
    CantReadOrdersLock,
    CantWriteOrdersLock,
    CantWriteContainerLock,
    CantReadContainerLock,
    NoMoreIngredient,
}
