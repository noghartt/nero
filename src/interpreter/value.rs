use std::ops;

#[derive(Debug)]
pub enum Value {
    Int(i64),
    Nil,
}

impl ops::Add<Value> for Value {
    type Output = Result<Value, ()>;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
            _ => Err(()),
        }
    }
}

impl ops::Sub<Value> for Value {
    type Output = Result<Value, ()>;

    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
            _ => Err(()),
        }
    }
}

impl ops::Neg for Value {
    type Output = Result<Value, ()>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Int(i) => Ok(Value::Int(-i)),
            _ => Err(()),
        }
    }
}
