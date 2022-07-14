use std::sync::Mutex;

use actix_web::{
    body::to_bytes,
    dev::ServiceResponse,
    test::{self, TestRequest},
    web::{self, service, Data},
    App,
};
use serde_json::json;

use crate::server::{check_if_joined, create_game, game_state, join_game, turn, AppData};

pub struct TestData {
    uri: String,
    test_request: TestRequest,
    expected_status_code: u16,
    expected_value: String,
}

impl TestData {
    pub fn new(
        uri: String,
        test_request: TestRequest,
        expected_status_code: u16,
        expected_value: String,
    ) -> TestData {
        return TestData {
            uri: uri,
            test_request: test_request,
            expected_status_code: expected_status_code,
            expected_value,
        };
    }
}

pub async fn make_requests(test_data_arr: Vec<TestData>) {
    let data = Data::new(Mutex::new(AppData::new()));
    let mut app = test::init_service(
        App::new()
            .app_data(data)
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(create_game)
            .service(join_game)
            .service(check_if_joined)
            .service(turn)
            .service(game_state),
    )
    .await;
    for test_data in test_data_arr {
        let response = test::call_service(
            &mut app,
            test_data.test_request.uri(&test_data.uri).to_request(),
        )
        .await;
        println!("{}", test_data.uri);
        assert_eq!(test_data.expected_status_code, response.status().as_u16());
        if (test_data.expected_value != "".to_string()) {
            assert_eq!(test_data.expected_value, get_body_as_string(response).await);
        }
    }
}

pub async fn get_body_as_string(response: ServiceResponse) -> String {
    let body = to_bytes(response.into_body()).await.unwrap().to_vec();
    String::from_utf8(body).unwrap()
}
pub enum Step {
    NoneTests = 0,
    CreateGame,
    JoinGame,
    CheckIfJoined,
    Turn00First,
    Turn01Second,
    Turn11First,
    Turn10Second,
    Turn22First,
}

pub fn get_slice_of_steps(step: Step, game_id: u128) -> Vec<TestData> {
    let mut test_vector = vec![];
    test_vector.push(TestData::new(
        "/create-game".to_string(),
        TestRequest::post(),
        201,
        game_id.to_string(),
    ));
    test_vector.push(TestData::new(
        "/join/random".to_string(),
        TestRequest::post(),
        200,
        game_id.to_string(),
    ));
    test_vector.push(TestData::new(
        format!("/check-if-joined/{}", game_id),
        TestRequest::get(),
        200,
        json!({ "player_joined": true }).to_string(),
    ));
    test_vector.push(TestData::new(
        format!("/turn/{}?x=0&y=0&turn_type=1", game_id),
        TestRequest::post(),
        200,
        json!({"field":[[1,0,0],[0,0,0],[0,0,0]],"winner":"NONE","turn":"SECOND"}).to_string(),
    ));
    test_vector.push(TestData::new(
        format!("/turn/{}?x=0&y=1&turn_type=-1", game_id),
        TestRequest::post(),
        200,
        json!({"field":[[1,0,0],[-1,0,0],[0,0,0]],"winner":"NONE","turn":"FIRST"}).to_string(),
    ));
    test_vector.push(TestData::new(
        format!("/turn/{}?x=1&y=1&turn_type=1", game_id),
        TestRequest::post(),
        200,
        json!({"field":[[1,0,0],[-1,1,0],[0,0,0]],"winner":"NONE","turn":"SECOND"}).to_string(),
    ));
    test_vector.push(TestData::new(
        format!("/turn/{}?x=1&y=0&turn_type=-1", game_id),
        TestRequest::post(),
        200,
        json!({"field":[[1,-1,0],[-1,1,0],[0,0,0]],"winner":"NONE","turn":"FIRST"}).to_string(),
    ));

    test_vector.push(TestData::new(
        format!("/turn/{}?x=2&y=2&turn_type=1", game_id),
        TestRequest::post(),
        200,
        json!({"field":[[1,-1,0],[-1,1,0],[0,0,1]],"winner":"FIRST","turn" :"SECOND"}).to_string(),
    ));
    for i in (step as usize)..test_vector.len() {
        test_vector.pop();
    }
    return test_vector;
}
