pub struct CurrencyValue {
    _value: f64,
}

impl CurrencyValue {
    pub fn new(value: f64) -> Self {
        Self { _value: value }
    }

    pub fn value(&self) -> &f64 {
        &self._value
    }

    pub fn formatted(&self) -> String {
        let mut prefix = String::from("$");
        prefix.push_str(&self._value.to_string());
        return prefix;
    }
}
