use crate::FixValue;

/// Allows to write FIX fields.
pub trait SetField<F> {
    fn set<'a, V>(&'a mut self, field: F, value: V)
    where
        V: FixValue<'a>,
    {
        self.set_with(field, value, <V::SerializeSettings as Default>::default())
    }

    fn set_with<'a, V>(&'a mut self, field: F, value: V, setting: V::SerializeSettings)
    where
        V: FixValue<'a>;
}
