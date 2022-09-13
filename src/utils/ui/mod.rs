#[derive(Debug, Clone)]
pub struct PickListContainer<T> {
    pub label: String,
    pub data: T,
}

impl<T> PickListContainer<T> {
    pub fn new(label: String, data: T) -> Self {
        Self { label, data }
    }
}

impl<T> ToString for PickListContainer<T> {
    fn to_string(&self) -> String {
        self.label.clone()
    }
}

impl<T: Eq> PartialEq<Self> for PickListContainer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<T: Eq> Eq for PickListContainer<T> {}
