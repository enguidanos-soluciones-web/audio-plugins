pub trait Reverse {
    type Output;

    fn reverse(self) -> Self::Output;
}

impl<A, B> Reverse for (A, B)
where
    A: Copy,
    B: Copy,
{
    type Output = (B, A);

    fn reverse(self) -> Self::Output {
        (self.1, self.0)
    }
}

pub trait CopyFillFromLeft {
    type Output;

    fn copy_fill_from_left(self) -> Self::Output;
}

impl<A, B> CopyFillFromLeft for (A, B)
where
    A: Copy,
    B: Copy,
{
    type Output = (A, A);

    fn copy_fill_from_left(self) -> Self::Output {
        (self.0, self.0)
    }
}

pub trait CopyFillFromRight {
    type Output;

    fn copy_fill_from_right(self) -> Self::Output;
}

impl<A, B> CopyFillFromRight for (A, B)
where
    A: Copy,
    B: Copy,
{
    type Output = (B, B);

    fn copy_fill_from_right(self) -> Self::Output {
        (self.1, self.1)
    }
}
