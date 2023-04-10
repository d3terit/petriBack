struct Arc{
    source: Place,
    target: Place,
}

impl Arc {
    fn new(source: Place, target: Place) -> Arc {
        Arc {
            source: source,
            target: target,
        }
    }

    fn get_source(&self) -> Place {
        self.source
    }

    fn get_target(&self) -> Place {
        self.target
    }
}