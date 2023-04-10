struct Node{
    id: String,
    name: String,
    x: i32,
    y: i32,
}

impl Node {
    fn new(id: String, name: String, x: i32, y: i32) -> Node {
        Node {
            id,
            name,
            x,
            y,
        }
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }
}