use chrono::{offset::Utc, DateTime};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(Option<bool>),
    Int(Option<i32>),
    Unsigned(Option<u32>),
    String(Option<String>),
    Bytes(Option<Vec<u8>>),
    DateTime(Option<DateTime<Utc>>),
}

impl TryInto<Option<String>> for Value {
    type Error = ();

    fn try_into(self) -> Result<Option<String>, Self::Error> {
        if let Value::String(s) = self {
            return Ok(s);
        }
        Err(())
    }
}

impl TryInto<String> for Value {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        if let Some(s) = self.try_into().unwrap() {
            return Ok(s);
        }
        Ok(Default::default())
    }
}

impl TryInto<Option<i32>> for Value {
    type Error = ();

    fn try_into(self) -> Result<Option<i32>, Self::Error> {
        if let Value::Int(i) = self {
            return Ok(i);
        }
        Err(())
    }
}

impl TryInto<i32> for Value {
    type Error = ();

    fn try_into(self) -> Result<i32, Self::Error> {
        if let Some(i) = self.try_into().unwrap() {
            return Ok(i);
        }
        Ok(Default::default())
    }
}

impl TryInto<Option<u32>> for Value {
    type Error = ();

    fn try_into(self) -> Result<Option<u32>, Self::Error> {
        match self {
            Value::Unsigned(u) => Ok(u),
            Value::Int(i) => Ok(u32::try_from(i.unwrap()).map(|u| Some(u)).map_err(|_| ())?),
            _ => Err(()),
        }
    }
}

impl TryInto<u32> for Value {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        if let Some(u) = self.try_into().unwrap() {
            return Ok(u);
        }
        Ok(Default::default())
    }
}

impl TryInto<Option<bool>> for Value {
    type Error = ();

    fn try_into(self) -> Result<Option<bool>, Self::Error> {
        if let Value::Bool(b) = self {
            return Ok(b);
        }
        Err(())
    }
}

impl TryInto<bool> for Value {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        if let Some(b) = self.try_into().unwrap() {
            return Ok(b);
        }
        Ok(Default::default())
    }
}

impl TryInto<Option<DateTime<Utc>>> for Value {
    type Error = ();

    fn try_into(self) -> Result<Option<DateTime<Utc>>, Self::Error> {
        if let Value::DateTime(dt) = self {
            return Ok(dt);
        }
        Err(())
    }
}

impl TryInto<DateTime<Utc>> for Value {
    type Error = ();

    fn try_into(self) -> Result<DateTime<Utc>, Self::Error> {
        if let Some(dt) = self.try_into().unwrap() {
            return Ok(dt);
        }
        Ok(Default::default())
    }
}

impl Into<Value> for i32 {
    fn into(self) -> Value {
        Value::Int(Some(self))
    }
}

impl Into<Value> for Option<i32> {
    fn into(self) -> Value {
        Value::Int(self)
    }
}

impl Into<Value> for u32 {
    fn into(self) -> Value {
        Value::Unsigned(Some(self))
    }
}

impl Into<Value> for Option<u32> {
    fn into(self) -> Value {
        Value::Unsigned(self)
    }
}

impl Into<Value> for String {
    fn into(self) -> Value {
        Value::String(Some(self))
    }
}

impl Into<Value> for Option<String> {
    fn into(self) -> Value {
        Value::String(self)
    }
}

impl Into<Value> for &str {
    fn into(self) -> Value {
        Value::String(Some(self.to_string()))
    }
}

impl Into<Value> for Option<&str> {
    fn into(self) -> Value {
        Value::String(match self {
            Some(s) => Some(s.to_string()),
            None => None,
        })
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::Bool(Some(self))
    }
}

impl Into<Value> for Option<bool> {
    fn into(self) -> Value {
        Value::Bool(self)
    }
}

impl Into<Value> for DateTime<Utc> {
    fn into(self) -> Value {
        Value::DateTime(Some(self))
    }
}

impl Into<Value> for Option<DateTime<Utc>> {
    fn into(self) -> Value {
        Value::DateTime(self)
    }
}

#[derive(Default, Debug)]
pub struct Values(pub Vec<Value>);

impl Values {
    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0);
    }
}
