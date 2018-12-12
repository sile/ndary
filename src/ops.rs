pub trait DotProduct<RHS = Self> {
    type Output;

    fn dot_product(self, rhs: RHS) -> Self::Output;
}
