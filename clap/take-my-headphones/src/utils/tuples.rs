pub trait Reverse {
    type Output;

    /// Swap the two elements of a tuple, returning `(B, A)` from `(A, B)`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!((1, 2).reverse(), (2, 1));
    /// ```
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

    /// Duplicate the left element of a tuple, returning `(A, A)` from `(A, B)`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!((1, 2).copy_fill_from_left(), (1, 1));
    /// ```
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

    /// Duplicate the right element of a tuple, returning `(B, B)` from `(A, B)`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!((1, 2).copy_fill_from_right(), (2, 2));
    /// ```
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
