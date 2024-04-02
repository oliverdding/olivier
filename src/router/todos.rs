use axum::routing::get;

use crate::{handler::todos, server::AppState};

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/v1/todos", get(todos::get_todos).post(todos::post_todos))
        .route(
            "/v1/todos/:id",
            get(todos::get_todo_by_id)
                .put(todos::put_todo_by_id)
                .delete(todos::delete_todo_by_id),
        )
}
