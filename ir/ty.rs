trait InternalTy {
    fn size(&self) -> Option<u32>;
    fn alignment(&self) -> u32;
}

#[derive(Debug, Clone, Copy)]
pub struct BasicTy {
    size: u32,
    alignment: u32,
}

macro_rules! basic_ty {
    ($name:ident, $size:expr, $align:expr) => {
        pub fn $name() -> Self {
            Self {
                size: $size,
                alignment: $align,
            }
        }
    };
}

impl BasicTy {
    basic_ty!(u8, 1, 1);
    basic_ty!(i8, 1, 1);
    basic_ty!(u16, 2, 2);
    basic_ty!(i16, 2, 2);
    basic_ty!(u32, 4, 4);
    basic_ty!(i32, 4, 4);
    basic_ty!(u64, 8, 8);
    basic_ty!(i64, 8, 8);
}

impl InternalTy for BasicTy {
    fn size(&self) -> Option<u32> {
        Some(self.size)
    }

    fn alignment(&self) -> u32 {
        self.alignment
    }
}

pub enum Ty {
  Basic(BasicTy),
  Array(BasicTy, u32),
  Pointer(BasicTy)
}
