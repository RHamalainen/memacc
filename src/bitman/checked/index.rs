/// Can index 8-bit unsigned integer.
pub trait IndexU8 {
    /// Index's value.
    fn value(self) -> usize;
}

/// Can index 32-bit unsigned integer.
pub trait IndexU32 {
    /// Index's value.
    fn value(self) -> usize;
}

/// Index.
pub struct I<const T: usize>;

#[macro_export]
macro_rules! index {
    ($indexor:literal) => {
        I::<$indexor>
    };
}

impl IndexU8 for I<0> {
    fn value(self) -> usize {
        0
    }
}
impl IndexU8 for I<1> {
    fn value(self) -> usize {
        1
    }
}
impl IndexU8 for I<2> {
    fn value(self) -> usize {
        2
    }
}
impl IndexU8 for I<3> {
    fn value(self) -> usize {
        3
    }
}
impl IndexU8 for I<4> {
    fn value(self) -> usize {
        4
    }
}
impl IndexU8 for I<5> {
    fn value(self) -> usize {
        5
    }
}
impl IndexU8 for I<6> {
    fn value(self) -> usize {
        6
    }
}
impl IndexU8 for I<7> {
    fn value(self) -> usize {
        7
    }
}
impl IndexU32 for I<0> {
    fn value(self) -> usize {
        0
    }
}
impl IndexU32 for I<1> {
    fn value(self) -> usize {
        1
    }
}
impl IndexU32 for I<2> {
    fn value(self) -> usize {
        2
    }
}
impl IndexU32 for I<3> {
    fn value(self) -> usize {
        3
    }
}
impl IndexU32 for I<4> {
    fn value(self) -> usize {
        4
    }
}
impl IndexU32 for I<5> {
    fn value(self) -> usize {
        5
    }
}
impl IndexU32 for I<6> {
    fn value(self) -> usize {
        6
    }
}
impl IndexU32 for I<7> {
    fn value(self) -> usize {
        7
    }
}
impl IndexU32 for I<8> {
    fn value(self) -> usize {
        8
    }
}
impl IndexU32 for I<9> {
    fn value(self) -> usize {
        9
    }
}
impl IndexU32 for I<10> {
    fn value(self) -> usize {
        10
    }
}
impl IndexU32 for I<11> {
    fn value(self) -> usize {
        11
    }
}
impl IndexU32 for I<12> {
    fn value(self) -> usize {
        12
    }
}
impl IndexU32 for I<13> {
    fn value(self) -> usize {
        13
    }
}
impl IndexU32 for I<14> {
    fn value(self) -> usize {
        14
    }
}
impl IndexU32 for I<15> {
    fn value(self) -> usize {
        15
    }
}
impl IndexU32 for I<16> {
    fn value(self) -> usize {
        16
    }
}
impl IndexU32 for I<17> {
    fn value(self) -> usize {
        17
    }
}
impl IndexU32 for I<18> {
    fn value(self) -> usize {
        18
    }
}
impl IndexU32 for I<19> {
    fn value(self) -> usize {
        19
    }
}
impl IndexU32 for I<20> {
    fn value(self) -> usize {
        20
    }
}
impl IndexU32 for I<21> {
    fn value(self) -> usize {
        21
    }
}
impl IndexU32 for I<22> {
    fn value(self) -> usize {
        22
    }
}
impl IndexU32 for I<23> {
    fn value(self) -> usize {
        23
    }
}
impl IndexU32 for I<24> {
    fn value(self) -> usize {
        24
    }
}
impl IndexU32 for I<25> {
    fn value(self) -> usize {
        25
    }
}
impl IndexU32 for I<26> {
    fn value(self) -> usize {
        26
    }
}
impl IndexU32 for I<27> {
    fn value(self) -> usize {
        27
    }
}
impl IndexU32 for I<28> {
    fn value(self) -> usize {
        28
    }
}
impl IndexU32 for I<29> {
    fn value(self) -> usize {
        29
    }
}
impl IndexU32 for I<30> {
    fn value(self) -> usize {
        30
    }
}
impl IndexU32 for I<31> {
    fn value(self) -> usize {
        31
    }
}
