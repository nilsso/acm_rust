pub trait Zero {
    const ZERO: Self;
}

impl Zero for i32 {
    const ZERO: i32 = 0;
}

pub trait One {
    const ONE: Self;
}

impl One for i32 {
    const ONE: i32 = 1;
}
