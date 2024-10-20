#[derive(Default, Clone, Debug, PartialEq)]
pub struct MemberInfoFlags {
    pub flag_bits: u16,
}

impl MemberInfoFlags {
    pub const MEMBER_TYPE_MASK: u16 = 1;

    pub const TYPE_CATEGORY_SHIFT: u16 = 1;
    pub const TYPE_CATEGORY_MASK: u16 = 15;

    pub const TYPE_CODE_SHIFT: u16 = 5;
    pub const TYPE_CODE_MASK: u16 = 31;

    pub const METADATA: u16 = 1 << 11;
    pub const HOMOGENEOUS: u16 = 1 << 12;
    pub const ALWAYS_PERSIST: u16 = 1 << 13;
    pub const EXPOSED: u16 = 1 << 13;
    pub const FLAGS_ENUM: u16 = 1 << 13;
    pub const LAYOUT_IMMUTABLE: u16 = 1 << 14;
    pub const BLITTABLE: u16 = 1 << 15;

    pub const fn new(flag_bits: u16) -> Self {
        Self { flag_bits }
    }

    pub fn exposed(&self) -> bool {
        self.flag_bits & Self::EXPOSED != 0
    }

    pub fn meta_field(&self) -> bool {
        self.flag_bits & Self::METADATA != 0
    }

    pub fn always_persist(&self) -> bool {
        self.flag_bits & Self::ALWAYS_PERSIST != 0
    }

    pub fn blittable(&self) -> bool {
        self.flag_bits & Self::BLITTABLE != 0
    }

    pub fn homogeneous(&self) -> bool {
        self.flag_bits & Self::HOMOGENEOUS != 0
    }

    pub fn layout_immutable(&self) -> bool {
        self.flag_bits & Self::LAYOUT_IMMUTABLE != 0
    }
}
