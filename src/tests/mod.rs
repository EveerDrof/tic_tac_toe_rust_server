mod helpers;

#[cfg(test)]
mod tests {

    use actix_web::test::TestRequest;
    use serde_json::json;

    use crate::server::GameData;

    use super::helpers::{get_slice_of_steps, make_requests, Step, TestData};
    #[actix_web::test]
    pub async fn create_game_should_return_201() {
        make_requests(get_slice_of_steps(Step::CreateGame, 0)).await;
    }
    #[actix_web::test]
    pub async fn create_join_random_return_game_id() {
        make_requests(get_slice_of_steps(Step::Turn22First, 0)).await;
    }
    #[actix_web::test]

    pub async fn get_game_state() {
        let mut tests = get_slice_of_steps(Step::JoinGame, 0);
        tests.push(TestData::new(
            "/game-state/0".to_string(),
            TestRequest::get(),
            200,
            json!({ "field": [[0,0,0],[0,0,0],[0,0,0]],"winner":"NONE","turn":"FIRST" })
                .to_string(),
        ));
        make_requests(tests).await;
    }
}
