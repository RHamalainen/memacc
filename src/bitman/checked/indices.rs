/// Can index 8-bit unsigned integer.
pub trait IndexRangeU8 {
    fn start(self) -> u8;
    fn end(self) -> u8;
}

/// Can index 32-bit unsigned integer.
pub trait IndexRangeU32 {
    fn start(self) -> u32;
    fn end(self) -> u32;
}

/// Index range.
#[derive(Clone, Copy)]
pub struct IR<const START: usize, const END: usize>;

#[macro_export]
macro_rules! indices {
    ($start:literal, $end:literal) => {};
}

impl IndexRangeU8 for IR<0, 0> {
    fn start(self) -> u8 {
        0
    }
    fn end(self) -> u8 {
        0
    }
}
impl IndexRangeU8 for IR<0, 1> {
    fn start(self) -> u8 {
        0
    }
    fn end(self) -> u8 {
        1
    }
}
impl IndexRangeU8 for IR<0, 2> {
    fn start(self) -> u8 {
        0
    }
    fn end(self) -> u8 {
        2
    }
}
impl IndexRangeU8 for IR<0, 3> {
    fn start(self) -> u8 {
        0
    }
    fn end(self) -> u8 {
        3
    }
}
impl IndexRangeU8 for IR<0, 4> {
    fn start(self) -> u8 {
        0
    }
    fn end(self) -> u8 {
        4
    }
}
impl IndexRangeU8 for IR<0, 5> {
    fn start(self) -> u8 {
        0
    }
    fn end(self) -> u8 {
        5
    }
}
impl IndexRangeU8 for IR<0, 6> {
    fn start(self) -> u8 {
        0
    }
    fn end(self) -> u8 {
        6
    }
}
impl IndexRangeU8 for IR<0, 7> {
    fn start(self) -> u8 {
        0
    }
    fn end(self) -> u8 {
        7
    }
}
impl IndexRangeU8 for IR<1, 1> {
    fn start(self) -> u8 {
        1
    }
    fn end(self) -> u8 {
        1
    }
}
impl IndexRangeU8 for IR<1, 2> {
    fn start(self) -> u8 {
        1
    }
    fn end(self) -> u8 {
        2
    }
}
impl IndexRangeU8 for IR<1, 3> {
    fn start(self) -> u8 {
        1
    }
    fn end(self) -> u8 {
        3
    }
}
impl IndexRangeU8 for IR<1, 4> {
    fn start(self) -> u8 {
        1
    }
    fn end(self) -> u8 {
        4
    }
}
impl IndexRangeU8 for IR<1, 5> {
    fn start(self) -> u8 {
        1
    }
    fn end(self) -> u8 {
        5
    }
}
impl IndexRangeU8 for IR<1, 6> {
    fn start(self) -> u8 {
        1
    }
    fn end(self) -> u8 {
        6
    }
}
impl IndexRangeU8 for IR<1, 7> {
    fn start(self) -> u8 {
        1
    }
    fn end(self) -> u8 {
        7
    }
}
impl IndexRangeU8 for IR<2, 2> {
    fn start(self) -> u8 {
        2
    }
    fn end(self) -> u8 {
        2
    }
}
impl IndexRangeU8 for IR<2, 3> {
    fn start(self) -> u8 {
        2
    }
    fn end(self) -> u8 {
        3
    }
}
impl IndexRangeU8 for IR<2, 4> {
    fn start(self) -> u8 {
        2
    }
    fn end(self) -> u8 {
        4
    }
}
impl IndexRangeU8 for IR<2, 5> {
    fn start(self) -> u8 {
        2
    }
    fn end(self) -> u8 {
        5
    }
}
impl IndexRangeU8 for IR<2, 6> {
    fn start(self) -> u8 {
        2
    }
    fn end(self) -> u8 {
        6
    }
}
impl IndexRangeU8 for IR<2, 7> {
    fn start(self) -> u8 {
        2
    }
    fn end(self) -> u8 {
        7
    }
}
impl IndexRangeU8 for IR<3, 3> {
    fn start(self) -> u8 {
        3
    }
    fn end(self) -> u8 {
        3
    }
}
impl IndexRangeU8 for IR<3, 4> {
    fn start(self) -> u8 {
        3
    }
    fn end(self) -> u8 {
        4
    }
}
impl IndexRangeU8 for IR<3, 5> {
    fn start(self) -> u8 {
        3
    }
    fn end(self) -> u8 {
        5
    }
}
impl IndexRangeU8 for IR<3, 6> {
    fn start(self) -> u8 {
        3
    }
    fn end(self) -> u8 {
        6
    }
}
impl IndexRangeU8 for IR<3, 7> {
    fn start(self) -> u8 {
        3
    }
    fn end(self) -> u8 {
        7
    }
}
impl IndexRangeU8 for IR<4, 4> {
    fn start(self) -> u8 {
        4
    }
    fn end(self) -> u8 {
        4
    }
}
impl IndexRangeU8 for IR<4, 5> {
    fn start(self) -> u8 {
        4
    }
    fn end(self) -> u8 {
        5
    }
}
impl IndexRangeU8 for IR<4, 6> {
    fn start(self) -> u8 {
        4
    }
    fn end(self) -> u8 {
        6
    }
}
impl IndexRangeU8 for IR<4, 7> {
    fn start(self) -> u8 {
        4
    }
    fn end(self) -> u8 {
        7
    }
}
impl IndexRangeU8 for IR<5, 5> {
    fn start(self) -> u8 {
        5
    }
    fn end(self) -> u8 {
        5
    }
}
impl IndexRangeU8 for IR<5, 6> {
    fn start(self) -> u8 {
        5
    }
    fn end(self) -> u8 {
        6
    }
}
impl IndexRangeU8 for IR<5, 7> {
    fn start(self) -> u8 {
        5
    }
    fn end(self) -> u8 {
        7
    }
}
impl IndexRangeU8 for IR<6, 6> {
    fn start(self) -> u8 {
        6
    }
    fn end(self) -> u8 {
        6
    }
}
impl IndexRangeU8 for IR<6, 7> {
    fn start(self) -> u8 {
        6
    }
    fn end(self) -> u8 {
        7
    }
}
impl IndexRangeU8 for IR<7, 7> {
    fn start(self) -> u8 {
        7
    }
    fn end(self) -> u8 {
        7
    }
}
impl IndexRangeU32 for IR<0, 0> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        0
    }
}
impl IndexRangeU32 for IR<0, 1> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        1
    }
}
impl IndexRangeU32 for IR<0, 2> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        2
    }
}
impl IndexRangeU32 for IR<0, 3> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        3
    }
}
impl IndexRangeU32 for IR<0, 4> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        4
    }
}
impl IndexRangeU32 for IR<0, 5> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        5
    }
}
impl IndexRangeU32 for IR<0, 6> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        6
    }
}
impl IndexRangeU32 for IR<0, 7> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        7
    }
}
impl IndexRangeU32 for IR<0, 8> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        8
    }
}
impl IndexRangeU32 for IR<0, 9> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<0, 10> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<0, 11> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<0, 12> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<0, 13> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<0, 14> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<0, 15> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<0, 16> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<0, 17> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<0, 18> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<0, 19> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<0, 20> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<0, 21> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<0, 22> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<0, 23> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<0, 24> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<0, 25> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<0, 26> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<0, 27> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<0, 28> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<0, 29> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<0, 30> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<0, 31> {
    fn start(self) -> u32 {
        0
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<1, 1> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        1
    }
}
impl IndexRangeU32 for IR<1, 2> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        2
    }
}
impl IndexRangeU32 for IR<1, 3> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        3
    }
}
impl IndexRangeU32 for IR<1, 4> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        4
    }
}
impl IndexRangeU32 for IR<1, 5> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        5
    }
}
impl IndexRangeU32 for IR<1, 6> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        6
    }
}
impl IndexRangeU32 for IR<1, 7> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        7
    }
}
impl IndexRangeU32 for IR<1, 8> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        8
    }
}
impl IndexRangeU32 for IR<1, 9> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<1, 10> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<1, 11> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<1, 12> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<1, 13> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<1, 14> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<1, 15> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<1, 16> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<1, 17> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<1, 18> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<1, 19> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<1, 20> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<1, 21> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<1, 22> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<1, 23> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<1, 24> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<1, 25> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<1, 26> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<1, 27> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<1, 28> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<1, 29> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<1, 30> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<1, 31> {
    fn start(self) -> u32 {
        1
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<2, 2> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        2
    }
}
impl IndexRangeU32 for IR<2, 3> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        3
    }
}
impl IndexRangeU32 for IR<2, 4> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        4
    }
}
impl IndexRangeU32 for IR<2, 5> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        5
    }
}
impl IndexRangeU32 for IR<2, 6> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        6
    }
}
impl IndexRangeU32 for IR<2, 7> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        7
    }
}
impl IndexRangeU32 for IR<2, 8> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        8
    }
}
impl IndexRangeU32 for IR<2, 9> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<2, 10> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<2, 11> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<2, 12> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<2, 13> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<2, 14> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<2, 15> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<2, 16> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<2, 17> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<2, 18> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<2, 19> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<2, 20> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<2, 21> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<2, 22> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<2, 23> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<2, 24> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<2, 25> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<2, 26> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<2, 27> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<2, 28> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<2, 29> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<2, 30> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<2, 31> {
    fn start(self) -> u32 {
        2
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<3, 3> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        3
    }
}
impl IndexRangeU32 for IR<3, 4> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        4
    }
}
impl IndexRangeU32 for IR<3, 5> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        5
    }
}
impl IndexRangeU32 for IR<3, 6> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        6
    }
}
impl IndexRangeU32 for IR<3, 7> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        7
    }
}
impl IndexRangeU32 for IR<3, 8> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        8
    }
}
impl IndexRangeU32 for IR<3, 9> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<3, 10> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<3, 11> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<3, 12> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<3, 13> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<3, 14> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<3, 15> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<3, 16> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<3, 17> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<3, 18> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<3, 19> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<3, 20> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<3, 21> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<3, 22> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<3, 23> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<3, 24> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<3, 25> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<3, 26> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<3, 27> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<3, 28> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<3, 29> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<3, 30> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<3, 31> {
    fn start(self) -> u32 {
        3
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<4, 4> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        4
    }
}
impl IndexRangeU32 for IR<4, 5> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        5
    }
}
impl IndexRangeU32 for IR<4, 6> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        6
    }
}
impl IndexRangeU32 for IR<4, 7> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        7
    }
}
impl IndexRangeU32 for IR<4, 8> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        8
    }
}
impl IndexRangeU32 for IR<4, 9> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<4, 10> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<4, 11> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<4, 12> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<4, 13> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<4, 14> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<4, 15> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<4, 16> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<4, 17> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<4, 18> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<4, 19> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<4, 20> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<4, 21> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<4, 22> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<4, 23> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<4, 24> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<4, 25> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<4, 26> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<4, 27> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<4, 28> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<4, 29> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<4, 30> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<4, 31> {
    fn start(self) -> u32 {
        4
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<5, 5> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        5
    }
}
impl IndexRangeU32 for IR<5, 6> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        6
    }
}
impl IndexRangeU32 for IR<5, 7> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        7
    }
}
impl IndexRangeU32 for IR<5, 8> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        8
    }
}
impl IndexRangeU32 for IR<5, 9> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<5, 10> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<5, 11> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<5, 12> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<5, 13> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<5, 14> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<5, 15> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<5, 16> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<5, 17> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<5, 18> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<5, 19> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<5, 20> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<5, 21> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<5, 22> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<5, 23> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<5, 24> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<5, 25> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<5, 26> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<5, 27> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<5, 28> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<5, 29> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<5, 30> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<5, 31> {
    fn start(self) -> u32 {
        5
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<6, 6> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        6
    }
}
impl IndexRangeU32 for IR<6, 7> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        7
    }
}
impl IndexRangeU32 for IR<6, 8> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        8
    }
}
impl IndexRangeU32 for IR<6, 9> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<6, 10> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<6, 11> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<6, 12> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<6, 13> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<6, 14> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<6, 15> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<6, 16> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<6, 17> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<6, 18> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<6, 19> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<6, 20> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<6, 21> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<6, 22> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<6, 23> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<6, 24> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<6, 25> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<6, 26> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<6, 27> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<6, 28> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<6, 29> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<6, 30> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<6, 31> {
    fn start(self) -> u32 {
        6
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<7, 7> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        7
    }
}
impl IndexRangeU32 for IR<7, 8> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        8
    }
}
impl IndexRangeU32 for IR<7, 9> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<7, 10> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<7, 11> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<7, 12> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<7, 13> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<7, 14> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<7, 15> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<7, 16> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<7, 17> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<7, 18> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<7, 19> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<7, 20> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<7, 21> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<7, 22> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<7, 23> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<7, 24> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<7, 25> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<7, 26> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<7, 27> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<7, 28> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<7, 29> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<7, 30> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<7, 31> {
    fn start(self) -> u32 {
        7
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<8, 8> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        8
    }
}
impl IndexRangeU32 for IR<8, 9> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<8, 10> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<8, 11> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<8, 12> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<8, 13> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<8, 14> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<8, 15> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<8, 16> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<8, 17> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<8, 18> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<8, 19> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<8, 20> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<8, 21> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<8, 22> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<8, 23> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<8, 24> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<8, 25> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<8, 26> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<8, 27> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<8, 28> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<8, 29> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<8, 30> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<8, 31> {
    fn start(self) -> u32 {
        8
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<9, 9> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        9
    }
}
impl IndexRangeU32 for IR<9, 10> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<9, 11> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<9, 12> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<9, 13> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<9, 14> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<9, 15> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<9, 16> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<9, 17> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<9, 18> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<9, 19> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<9, 20> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<9, 21> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<9, 22> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<9, 23> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<9, 24> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<9, 25> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<9, 26> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<9, 27> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<9, 28> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<9, 29> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<9, 30> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<9, 31> {
    fn start(self) -> u32 {
        9
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<10, 10> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        10
    }
}
impl IndexRangeU32 for IR<10, 11> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<10, 12> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<10, 13> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<10, 14> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<10, 15> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<10, 16> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<10, 17> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<10, 18> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<10, 19> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<10, 20> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<10, 21> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<10, 22> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<10, 23> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<10, 24> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<10, 25> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<10, 26> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<10, 27> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<10, 28> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<10, 29> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<10, 30> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<10, 31> {
    fn start(self) -> u32 {
        10
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<11, 11> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        11
    }
}
impl IndexRangeU32 for IR<11, 12> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<11, 13> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<11, 14> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<11, 15> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<11, 16> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<11, 17> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<11, 18> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<11, 19> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<11, 20> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<11, 21> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<11, 22> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<11, 23> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<11, 24> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<11, 25> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<11, 26> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<11, 27> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<11, 28> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<11, 29> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<11, 30> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<11, 31> {
    fn start(self) -> u32 {
        11
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<12, 12> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        12
    }
}
impl IndexRangeU32 for IR<12, 13> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<12, 14> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<12, 15> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<12, 16> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<12, 17> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<12, 18> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<12, 19> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<12, 20> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<12, 21> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<12, 22> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<12, 23> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<12, 24> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<12, 25> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<12, 26> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<12, 27> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<12, 28> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<12, 29> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<12, 30> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<12, 31> {
    fn start(self) -> u32 {
        12
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<13, 13> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        13
    }
}
impl IndexRangeU32 for IR<13, 14> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<13, 15> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<13, 16> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<13, 17> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<13, 18> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<13, 19> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<13, 20> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<13, 21> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<13, 22> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<13, 23> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<13, 24> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<13, 25> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<13, 26> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<13, 27> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<13, 28> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<13, 29> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<13, 30> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<13, 31> {
    fn start(self) -> u32 {
        13
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<14, 14> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        14
    }
}
impl IndexRangeU32 for IR<14, 15> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<14, 16> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<14, 17> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<14, 18> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<14, 19> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<14, 20> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<14, 21> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<14, 22> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<14, 23> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<14, 24> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<14, 25> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<14, 26> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<14, 27> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<14, 28> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<14, 29> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<14, 30> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<14, 31> {
    fn start(self) -> u32 {
        14
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<15, 15> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        15
    }
}
impl IndexRangeU32 for IR<15, 16> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<15, 17> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<15, 18> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<15, 19> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<15, 20> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<15, 21> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<15, 22> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<15, 23> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<15, 24> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<15, 25> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<15, 26> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<15, 27> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<15, 28> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<15, 29> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<15, 30> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<15, 31> {
    fn start(self) -> u32 {
        15
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<16, 16> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        16
    }
}
impl IndexRangeU32 for IR<16, 17> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<16, 18> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<16, 19> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<16, 20> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<16, 21> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<16, 22> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<16, 23> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<16, 24> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<16, 25> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<16, 26> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<16, 27> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<16, 28> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<16, 29> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<16, 30> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<16, 31> {
    fn start(self) -> u32 {
        16
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<17, 17> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        17
    }
}
impl IndexRangeU32 for IR<17, 18> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<17, 19> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<17, 20> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<17, 21> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<17, 22> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<17, 23> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<17, 24> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<17, 25> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<17, 26> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<17, 27> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<17, 28> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<17, 29> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<17, 30> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<17, 31> {
    fn start(self) -> u32 {
        17
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<18, 18> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        18
    }
}
impl IndexRangeU32 for IR<18, 19> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<18, 20> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<18, 21> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<18, 22> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<18, 23> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<18, 24> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<18, 25> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<18, 26> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<18, 27> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<18, 28> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<18, 29> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<18, 30> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<18, 31> {
    fn start(self) -> u32 {
        18
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<19, 19> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        19
    }
}
impl IndexRangeU32 for IR<19, 20> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<19, 21> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<19, 22> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<19, 23> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<19, 24> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<19, 25> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<19, 26> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<19, 27> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<19, 28> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<19, 29> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<19, 30> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<19, 31> {
    fn start(self) -> u32 {
        19
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<20, 20> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        20
    }
}
impl IndexRangeU32 for IR<20, 21> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<20, 22> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<20, 23> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<20, 24> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<20, 25> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<20, 26> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<20, 27> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<20, 28> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<20, 29> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<20, 30> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<20, 31> {
    fn start(self) -> u32 {
        20
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<21, 21> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        21
    }
}
impl IndexRangeU32 for IR<21, 22> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<21, 23> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<21, 24> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<21, 25> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<21, 26> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<21, 27> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<21, 28> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<21, 29> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<21, 30> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<21, 31> {
    fn start(self) -> u32 {
        21
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<22, 22> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        22
    }
}
impl IndexRangeU32 for IR<22, 23> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<22, 24> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<22, 25> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<22, 26> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<22, 27> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<22, 28> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<22, 29> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<22, 30> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<22, 31> {
    fn start(self) -> u32 {
        22
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<23, 23> {
    fn start(self) -> u32 {
        23
    }
    fn end(self) -> u32 {
        23
    }
}
impl IndexRangeU32 for IR<23, 24> {
    fn start(self) -> u32 {
        23
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<23, 25> {
    fn start(self) -> u32 {
        23
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<23, 26> {
    fn start(self) -> u32 {
        23
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<23, 27> {
    fn start(self) -> u32 {
        23
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<23, 28> {
    fn start(self) -> u32 {
        23
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<23, 29> {
    fn start(self) -> u32 {
        23
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<23, 30> {
    fn start(self) -> u32 {
        23
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<23, 31> {
    fn start(self) -> u32 {
        23
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<24, 24> {
    fn start(self) -> u32 {
        24
    }
    fn end(self) -> u32 {
        24
    }
}
impl IndexRangeU32 for IR<24, 25> {
    fn start(self) -> u32 {
        24
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<24, 26> {
    fn start(self) -> u32 {
        24
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<24, 27> {
    fn start(self) -> u32 {
        24
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<24, 28> {
    fn start(self) -> u32 {
        24
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<24, 29> {
    fn start(self) -> u32 {
        24
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<24, 30> {
    fn start(self) -> u32 {
        24
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<24, 31> {
    fn start(self) -> u32 {
        24
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<25, 25> {
    fn start(self) -> u32 {
        25
    }
    fn end(self) -> u32 {
        25
    }
}
impl IndexRangeU32 for IR<25, 26> {
    fn start(self) -> u32 {
        25
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<25, 27> {
    fn start(self) -> u32 {
        25
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<25, 28> {
    fn start(self) -> u32 {
        25
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<25, 29> {
    fn start(self) -> u32 {
        25
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<25, 30> {
    fn start(self) -> u32 {
        25
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<25, 31> {
    fn start(self) -> u32 {
        25
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<26, 26> {
    fn start(self) -> u32 {
        26
    }
    fn end(self) -> u32 {
        26
    }
}
impl IndexRangeU32 for IR<26, 27> {
    fn start(self) -> u32 {
        26
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<26, 28> {
    fn start(self) -> u32 {
        26
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<26, 29> {
    fn start(self) -> u32 {
        26
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<26, 30> {
    fn start(self) -> u32 {
        26
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<26, 31> {
    fn start(self) -> u32 {
        26
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<27, 27> {
    fn start(self) -> u32 {
        27
    }
    fn end(self) -> u32 {
        27
    }
}
impl IndexRangeU32 for IR<27, 28> {
    fn start(self) -> u32 {
        27
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<27, 29> {
    fn start(self) -> u32 {
        27
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<27, 30> {
    fn start(self) -> u32 {
        27
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<27, 31> {
    fn start(self) -> u32 {
        27
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<28, 28> {
    fn start(self) -> u32 {
        28
    }
    fn end(self) -> u32 {
        28
    }
}
impl IndexRangeU32 for IR<28, 29> {
    fn start(self) -> u32 {
        28
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<28, 30> {
    fn start(self) -> u32 {
        28
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<28, 31> {
    fn start(self) -> u32 {
        28
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<29, 29> {
    fn start(self) -> u32 {
        29
    }
    fn end(self) -> u32 {
        29
    }
}
impl IndexRangeU32 for IR<29, 30> {
    fn start(self) -> u32 {
        29
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<29, 31> {
    fn start(self) -> u32 {
        29
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<30, 30> {
    fn start(self) -> u32 {
        30
    }
    fn end(self) -> u32 {
        30
    }
}
impl IndexRangeU32 for IR<30, 31> {
    fn start(self) -> u32 {
        30
    }
    fn end(self) -> u32 {
        31
    }
}
impl IndexRangeU32 for IR<31, 31> {
    fn start(self) -> u32 {
        31
    }
    fn end(self) -> u32 {
        31
    }
}
