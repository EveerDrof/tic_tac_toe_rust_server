use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self, Data},
    App, HttpResponse, HttpResponseBuilder, HttpServer, Responder,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::{HashMap, LinkedList},
    io::Error,
    result,
    sync::Mutex,
    vec,
};

pub struct AppData {
    waiting_games: LinkedList<u128>,
    started_games: HashMap<u128, GameData>,
    last_game_id: u128,
}
#[derive(Deserialize)]
pub struct GameData {
    field: Vec<Vec<i8>>,
    winner: String,
    turn: String,
}
impl GameData {
    pub fn new() -> GameData {
        let mut field = vec![];
        for i in 0..3 {
            let mut inner_vec = vec![];
            for i in 0..3 {
                inner_vec.push(0);
            }
            field.push(inner_vec);
        }
        GameData {
            field,
            winner: "NONE".to_string(),
            turn: "FIRST".to_string(),
        }
    }
    pub fn to_json(&self) -> Value {
        json!({"field":self.field,"winner":self.winner,"turn":self.turn})
    }
}

impl AppData {
    pub fn new() -> AppData {
        return AppData {
            waiting_games: LinkedList::new(),
            started_games: HashMap::new(),
            last_game_id: 0,
        };
    }
    pub fn add_new_game(&mut self) -> u128 {
        self.waiting_games.push_back(self.last_game_id);
        self.last_game_id += 1;
        return self.last_game_id - 1;
    }
    pub fn get_game_id(&mut self) -> u128 {
        *self.waiting_games.front().unwrap()
    }
    pub fn pop_first_waiting_game(&mut self) -> u128 {
        let first_waiting_game = self.waiting_games.pop_front().unwrap();
        self.started_games
            .insert(first_waiting_game, GameData::new());
        return first_waiting_game;
    }
    pub fn purge_games_list(&mut self) {
        self.waiting_games = LinkedList::new();
    }
    pub fn check_if_game_is_started(&self, game_id: u128) -> bool {
        !self.started_games.get(&game_id).is_none()
    }
}

#[post("/create-game")]
async fn create_game(data: Data<Mutex<AppData>>) -> impl Responder {
    let mut app_data = data.lock().unwrap();
    HttpResponse::Created().body(format!("{}", app_data.add_new_game()))
    // web::Json(json!({ "temperature": 42.3,"name":"{name}" }))
}
#[post("/join/{game_id}")]
async fn join_game(data: Data<Mutex<AppData>>, game_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", data.lock().unwrap().pop_first_waiting_game()))
}
#[get("/check-if-joined/{game_id}")]
async fn check_if_joined(data: Data<Mutex<AppData>>, game_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(
        json!({"player_joined":
                data.lock()
                    .unwrap()
                    .check_if_game_is_started(game_id.parse::<u128>().unwrap())
        })
        .to_string(),
    )
}
#[derive(Deserialize)]
struct TurnData {
    x: String,
    y: String,
    turn_type: String,
}
fn update_game_state(game: &mut GameData) {
    let mut winner = "NONE";
    for i in 0..3 {
        let mut signed = 0;
        for k in 0..3 {
            signed += game.field[i][k];
        }
        if signed == 3 {
            winner = "FIRST";
        } else if signed == -3 {
            winner = "SECOND";
        }
    }
    for i in 0..3 {
        let mut signed = 0;
        for k in 0..3 {
            signed += game.field[k][i];
        }
        if signed == 3 {
            winner = "FIRST";
        } else if signed == -3 {
            winner = "SECOND";
        }
    }
    let mut signed = 0;
    for i in 0..3 {
        signed += game.field[i][i];
    }
    if signed == 3 {
        winner = "FIRST";
    } else if signed == -3 {
        winner = "SECOND";
    }
    game.winner = winner.to_string();
    if (game.turn == "FIRST") {
        game.turn = "SECOND".to_string();
    } else {
        game.turn = "FIRST".to_string();
    }
}

#[post("/turn/{game_id}")]
async fn turn(
    data: Data<Mutex<AppData>>,
    game_id: web::Path<String>,
    turn_data: web::Query<TurnData>,
) -> impl Responder {
    let mut app_data = data.lock().unwrap();
    let game: &mut GameData = app_data
        .started_games
        .get_mut(&game_id.parse::<u128>().unwrap())
        .unwrap();
    let (x, y) = (
        turn_data.x.parse::<usize>().unwrap(),
        turn_data.y.parse::<usize>().unwrap(),
    );
    let turn_type = turn_data.turn_type.parse::<i8>().unwrap();
    game.field[y][x] = turn_type;
    update_game_state(game);
    return HttpResponse::Ok().body(game.to_json().to_string());
}
#[get("/game-state/{game_id}")]
async fn game_state(data: Data<Mutex<AppData>>, game_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(
        data.lock()
            .unwrap()
            .started_games
            .get(&game_id.parse::<u128>().unwrap())
            .unwrap()
            .to_json()
            .to_string(),
    )
}
pub async fn server() -> Result<(), Error> {
    let data = Data::new(Mutex::new(AppData::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/server", web::get().to(|| async { "Hello World!" }))
            .service(create_game)
            .service(join_game)
            .service(check_if_joined)
            .service(turn)
            .service(game_state)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 1111))?
    .run()
    .await
}
