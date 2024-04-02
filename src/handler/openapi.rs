use crate::dto::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // server
        crate::handler::server::health,
        crate::handler::server::state,
        // todos
        crate::handler::todos::get_todos,
        crate::handler::todos::post_todos,
        crate::handler::todos::get_todo_by_id,
        crate::handler::todos::put_todo_by_id,
        crate::handler::todos::delete_todo_by_id,
    ),
    components(
        schemas(
            ServiceStatusResponse,
            ErrorResponse,
            NewTodoRequest,
            UpdateTodoRequest,
            TodoResponse,
            TodosResponse,
        )
    ),
    tags(
        (name = "crate::handler::server", description = "server routers"),
        (name = "crate::handler::todos", description = "todos routers"),
    ),
)]
pub struct ApiDoc;
