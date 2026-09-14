/// Boolean like 32 bit integer wrapper
#[derive(Debug, Clone, Copy)]
pub struct Bool32(pub u32);

impl From<Bool32> for bool {
    fn from(value: Bool32) -> Self {
        value.0 != 0
    }
}

impl From<bool> for Bool32 {
    fn from(value: bool) -> Self {
        Self(value as u32)
    }
}

impl std::cmp::PartialEq<Self> for Bool32 {
    fn eq(&self, other: &Self) -> bool {
        bool::from(*self) == bool::from(*other)
    }
}

impl std::cmp::PartialEq<bool> for Bool32 {
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl std::cmp::PartialEq<Bool32> for bool {
    fn eq(&self, other: &Bool32) -> bool {
        bool::from(*other) == *self
    }
}
