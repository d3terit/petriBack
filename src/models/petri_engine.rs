use crate::models::{node::Node, arc::Arc};

struct PetriEngine {
    places: Vec<Node>,
    transitions: Vec<Node>,
    arcs: Vec<Arc>,
    initial_marking: HashMap<String, u32>,
}

impl PetriEngine {
    fn add() -> PetriEngine {
        PetriEngine {
            lugares: vec![0, 0, 0],
            transiciones: vec![
                vec![1, 0, 1],
                vec![0, 1, 1],
            ],
        }
    }

    fn shoot(&mut self, transicion: usize) -> Result<(), &'static str> {
        let marcado = self.lugares.clone();

        for i in 0..self.transiciones[transicion].len() {
            let token = self.transiciones[transicion][i];
            if token > marcado[i] {
                return Err("No hay suficientes tokens en el lugar");
            }
        }

        for i in 0..self.transiciones[transicion].len() {
            let token = self.transiciones[transicion][i];
            self.lugares[i] -= token;
        }

        Ok(())
    }
}
