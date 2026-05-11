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

pub trait FillFromLeft {
    type Output;

    fn fill_from_left(self) -> Self::Output;
}

impl<A, B> FillFromLeft for (A, B)
where
    A: Copy,
    B: Copy,
{
    type Output = (A, A);

    fn fill_from_left(self) -> Self::Output {
        (self.0, self.0)
    }
}

pub trait FillFromRight {
    type Output;

    fn fill_from_right(self) -> Self::Output;
}

impl<A, B> FillFromRight for (A, B)
where
    A: Copy,
    B: Copy,
{
    type Output = (B, B);

    fn fill_from_right(self) -> Self::Output {
        (self.1, self.1)
    }
}
