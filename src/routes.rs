use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use rocket::Route;
use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Api is running"
}

#[derive(Debug, Deserialize, Serialize)]
struct PetriNet {
    class: String,
    nodeKeyProperty: String,
    nodeDataArray: Vec<NodeData>,
    linkDataArray: Vec<LinkData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NodeData {
    id: i32,
    loc: String,
    text: String,
    category: String,
    tokens: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LinkData {
    from: i32,
    to: i32,
}

#[post("/check_transitions", data = "<petri_net_json>")]
fn check_transitions(petri_net_json: Json<PetriNet>) -> Json<Vec<String>> {
    let mut enabled_transitions = Vec::new();
    let petri_net = petri_net_json.into_inner();
    for node_data in &petri_net.nodeDataArray {
        if node_data.category == "transicion" {
            enabled_transitions.push(node_data.text.clone());
        }
    }
    Json(enabled_transitions)
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        check_transitions,
    ]
}