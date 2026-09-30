pub struct PhysicsModel {
    pub formulas_array: Vec<Formula>,
}

impl PhysicsModel {
    pub fn new(formulas_array: Vec<Formula>) -> Self {
        PhysicsModel {
            formulas_array,
        }
    }

    pub fn add(&mut self, formula: Formula) {
        self.formulas_array.push(formula);
    }

    pub fn remove(&mut self, index: usize) -> Option<Formula> {
        if index < self.formulas_array.len() {
            Some(self.formulas_array.remove(index))
        } else {
            None
        }
    }

    pub fn get_array(&self) -> &Vec<Formula> {
        &self.formulas_array
    }

    pub fn get_formula(&self, index: usize) -> Option<&Formula> {
        self.formulas_array.get(index)
    }
    pub fn upadate(){
        
    }
}
