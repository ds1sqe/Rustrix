pub trait AbstractDomain {
    fn bottom() -> Self;
    fn is_bottom() -> Self;

    fn top() -> Self;
    fn is_top() -> Self;

    fn join_with(&mut self, rhs: &Self);

    fn meet_with(&mut self, rhs: &Self);

    fn widen_with(&mut self, rhs: &Self);

    fn narrow_with(&mut self, rhs: &Self);
}
