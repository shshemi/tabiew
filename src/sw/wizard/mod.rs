pub mod importer;

#[derive(Debug)]
pub enum PickerValue<P, V> {
    Picker(P),
    Value(V),
}

impl<P, V> PickerValue<P, V> {
    pub fn new(picker: P) -> Self {
        Self::Picker(picker)
    }

    pub fn picked(&self) -> bool {
        matches!(self, PickerValue::Value(_))
    }

    pub fn value(&self) -> Option<&V> {
        match self {
            PickerValue::Picker(_) => None,
            PickerValue::Value(value) => Some(value),
        }
    }

    pub fn into_value(self) -> Result<V, Self> {
        match self {
            PickerValue::Picker(_) => Err(self),
            PickerValue::Value(v) => Ok(v),
        }
    }
}

impl<P, V> Default for PickerValue<P, V>
where
    P: Default,
{
    fn default() -> Self {
        Self::Picker(Default::default())
    }
}
