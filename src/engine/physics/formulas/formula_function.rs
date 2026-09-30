pub enum FormulaFunction{
    FreeFallVelocity(FreeFallVelocity),
}
impl FormulaFunction {
    let formula = FormulaFunction::FreeFallVelocity(
    FreeFallVelocity::Displacement
);
    pub fn calculate(&self) -> f64 {
        match self {
            FormulaFunction::FreeFallVelocity(formula) => {
                formula.calculate()
            }
        }
    }
}