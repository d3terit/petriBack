use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use rocket::Route;
use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Api is running"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PetriNet {
    class: String,
    nodeKeyProperty: String,
    nodeDataArray: Vec<NodeData>,
    linkDataArray: Vec<LinkData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct NodeData {
    id: i32,
    loc: String,
    text: String,
    category: String,
    tokens: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LinkData {
    from: i32,
    to: i32,
}

fn has_input_links(node_data: &NodeData, petri_net: &PetriNet) -> bool {
    for link in &petri_net.linkDataArray {
        if link.to == node_data.id {
            return true;
        }
    }
    false
}

fn has_enough_tokens(transicion: &NodeData, petri_net: &PetriNet) -> bool {
    if !has_input_links(transicion, petri_net) {
        return false;
    }
    let mut has_enough_tokens = true;
    for node in &petri_net.nodeDataArray {
        if node.category == "lugar" {
            let mut links = 0;
            for link in &petri_net.linkDataArray {
                if link.from == node.id && link.to == transicion.id {
                    links += 1;
                }
            }
            if node.tokens.unwrap() < links {
                has_enough_tokens = false;
            }
        }
    }
    has_enough_tokens
}

#[post("/check_transitions", data = "<petri_net_json>")]
fn check_transitions(petri_net_json: Json<PetriNet>) -> Json<Vec<i32>> {
    let mut enabled_transitions = Vec::new();
    let petri_net = petri_net_json.into_inner();
    for node_data in &petri_net.nodeDataArray {
        if node_data.category == "transicion" && has_enough_tokens(&node_data, &petri_net) {
            enabled_transitions.push(node_data.id.clone());
        }
    }
    Json(enabled_transitions)
}

fn find_transition(id: i32, petri_net: &PetriNet) -> Option<&NodeData> {
    for node in &petri_net.nodeDataArray {
        if node.category == "transicion" && node.id == id {
            return Some(node);
        }
    }
    None
}

#[post("/fire_transition/<id>", data = "<petri_net_json>")]
fn fire_transition(id: i32, petri_net_json: Json<PetriNet>) -> Json<PetriNet> {
    let petri_net = petri_net_json.into_inner();
    let transition = find_transition(id, &petri_net).unwrap();
    if !has_enough_tokens(transition, &petri_net) {
        return Json(petri_net);
    }
    let mut node_data_array = petri_net.nodeDataArray.to_vec();
    for i in 0..petri_net.nodeDataArray.len() {
        if petri_net.nodeDataArray[i].category == "lugar" {
            let mut links = 0;
            for link in &petri_net.linkDataArray {
                if link.from == petri_net.nodeDataArray[i].id && link.to == transition.id {
                    links += 1;
                }
                if link.from == transition.id && link.to == petri_net.nodeDataArray[i].id {
                    links -= 1;
                }
            }
            node_data_array[i].tokens = Some(petri_net.nodeDataArray[i].tokens.unwrap() - links);
        }
    }
    Json(PetriNet {
        class: petri_net.class,
        nodeKeyProperty: petri_net.nodeKeyProperty,
        nodeDataArray: node_data_array,
        linkDataArray: petri_net.linkDataArray,
    })
}

#[post("/simulate", data = "<petri_net_json>")]
fn simulate(petri_net_json: Json<PetriNet>) -> Json<PetriNet> {
    let mut petri_net = petri_net_json.into_inner();
    let enabled_transitions = check_transitions(Json(petri_net.clone())).into_inner();
    for transition in enabled_transitions {
        petri_net = fire_transition(transition, Json(petri_net.clone())).into_inner();
    }
    Json(petri_net)
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        check_transitions,
        fire_transition,
        simulate
    ]
}