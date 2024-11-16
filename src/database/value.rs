use super::Error;

/// Simplifies the data conversion between database and application
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(Option<bool>),
    Int(Option<i32>),
    Int64(Option<i64>),
    Uint(Option<u32>),
    Uint64(Option<u64>),
    String(Option<String>),
    Bytes(Option<Vec<u8>>),
    DateTime(Option<chrono::DateTime<chrono::Utc>>),
}

// ===
// TryFrom<Value> to Option<T>

impl TryFrom<Value> for Option<bool> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(b) => Ok(b),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for Option<i32> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(i),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for Option<i64> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int64(i) => Ok(i),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for Option<u32> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => Ok(u),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for Option<u64> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint64(u) => Ok(u),
            Value::Int64(Some(i)) => u64::try_from(i).map(Some).map_err(|_| Error::ValueParsing),
            Value::Int64(_) => Ok(None),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for Option<usize> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Option::<u64>::try_from(value).and_then(|o| match o {
            Some(u) => usize::try_from(u)
                .map(Some)
                .map_err(|_| Error::ValueParsing),
            _ => Ok(None),
        })
    }
}

impl TryFrom<Value> for Option<String> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for Option<Vec<u8>> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(bs) => Ok(bs),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for Option<chrono::DateTime<chrono::Utc>> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::DateTime(dt) => Ok(dt),
            _ => Err(Error::ValueParsing),
        }
    }
}

// ===
// TryFrom<Value> to T

impl TryFrom<Value> for bool {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(Some(b)) => Ok(b),
            Value::Bool(_) => Ok(Default::default()),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for i32 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(Some(i)) => Ok(i),
            Value::Int(_) => Ok(Default::default()),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int64(Some(i)) => Ok(i),
            Value::Int64(_) => Ok(Default::default()),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for u32 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(Some(u)) => Ok(u),
            Value::Uint(_) => Ok(Default::default()),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint64(Some(u)) => Ok(u),
            Value::Int64(Some(i)) => u64::try_from(i).map_err(|_| Error::ValueParsing),
            Value::Uint64(_) | Value::Int64(_) => Ok(Default::default()),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for usize {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        u64::try_from(value).and_then(|u| usize::try_from(u).map_err(|_| Error::ValueParsing))
    }
}

impl TryFrom<Value> for String {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(Some(s)) => Ok(s),
            Value::String(_) => Ok(Default::default()),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(Some(b)) => Ok(b),
            Value::Bytes(_) => Ok(Default::default()),
            _ => Err(Error::ValueParsing),
        }
    }
}

impl TryFrom<Value> for chrono::DateTime<chrono::Utc> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::DateTime(Some(dt)) => Ok(dt),
            Value::DateTime(_) => Ok(Default::default()),
            _ => Err(Error::ValueParsing),
        }
    }
}

// ===
// From<Option<T>> to Value

impl From<Option<bool>> for Value {
    fn from(value: Option<bool>) -> Self {
        Value::Bool(value)
    }
}

impl From<Option<String>> for Value {
    fn from(value: Option<String>) -> Self {
        Value::String(value)
    }
}

impl<'a> From<Option<&'a str>> for Value {
    fn from(value: Option<&'a str>) -> Self {
        Value::String(value.map(|s| s.to_string()))
    }
}

impl From<Option<i32>> for Value {
    fn from(value: Option<i32>) -> Self {
        Value::Int(value)
    }
}

impl From<Option<i64>> for Value {
    fn from(value: Option<i64>) -> Self {
        Value::Int64(value)
    }
}

impl From<Option<u32>> for Value {
    fn from(value: Option<u32>) -> Self {
        Value::Uint(value)
    }
}

impl From<Option<u64>> for Value {
    fn from(value: Option<u64>) -> Self {
        Value::Uint64(value)
    }
}

impl From<Option<usize>> for Value {
    fn from(value: Option<usize>) -> Self {
        value
            .map(|u| u64::try_from(u).expect("usize to u64"))
            .into()
    }
}

impl From<Option<Vec<u8>>> for Value {
    fn from(value: Option<Vec<u8>>) -> Self {
        Value::Bytes(value)
    }
}

impl From<Option<chrono::DateTime<chrono::Utc>>> for Value {
    fn from(value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        Value::DateTime(value)
    }
}

// ===
// From<T> to Value

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(Some(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(Some(value))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Value::String(Some(value.to_string()))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(Some(value))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int64(Some(value))
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Uint(Some(value))
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Uint64(Some(value))
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        u64::try_from(value).expect("usize to u64").into()
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Bytes(Some(value))
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Value {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Value::DateTime(Some(value))
    }
}

// ===
// TryFrom<T> to Value

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_into() {
        let v = Value::String(Some("".to_string()));
        println!("{:?}", String::try_from(v));
    }
}
