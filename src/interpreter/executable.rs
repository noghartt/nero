use super::value::Value;

pub trait Executable {
    fn run(&self) -> Result<Value, ()>;
}
