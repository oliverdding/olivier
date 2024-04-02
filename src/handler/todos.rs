use axum::extract::Path;
use axum::{extract::State, Json};
use axum_extra::extract::WithRejection;
use chrono::Utc;
use garde::Validate;
use http::StatusCode;

use entity::todos::ActiveModel as TodosActiveModel;
use entity::todos::Entity as TodosEntity;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

use crate::dto::{TodoResponse, UpdateTodoRequest};
use crate::{
    dto::{NewTodoRequest, TodosResponse},
    error::{AppResult, ServiceError},
    server::AppState,
};

#[utoipa::path(
    get,
    path = "/api/v1/todos",
    responses(
        (status = 200, description = "get all todos", body = [TodosResponse]),
        (status = 500, description = "database error", body = [ErrorResponse]),
    )
)]
pub async fn get_todos(State(state): State<AppState>) -> AppResult<TodosResponse> {
    let res = TodosEntity::find().all(&*state.database).await?;

    let todos = TodosResponse {
        todos: res.into_iter().map(|x| x.into()).collect(),
    };
    Ok(todos)
}

#[utoipa::path(
    post,
    request_body = NewTodoRequest,
    path = "/api/v1/todos",
    responses(
        (status = 201, description = "create todo", body = [TodoResponse]),
        (status = 400, description = "invalid request", body = [ErrorResponse]),
        (status = 422, description = "lack of necessary fields", body = [ErrorResponse]),
        (status = 500, description = "database error", body = [ErrorResponse]),
    )
)]
pub async fn post_todos(
    State(state): State<AppState>,
    WithRejection(Json(payload), _): WithRejection<Json<NewTodoRequest>, ServiceError>,
) -> AppResult<(StatusCode, TodoResponse)> {
    payload.validate(&())?;

    let todo = TodosActiveModel {
        body: ActiveValue::set(payload.body),
        complated: ActiveValue::set(payload.complated.unwrap_or_default()),
        ..Default::default()
    };

    let res = todo.insert(&*state.database).await?;

    Ok((StatusCode::CREATED, res.into()))
}

#[utoipa::path(
    get,
    path = "/api/v1/todos/{id}",
    responses(
        (status = 200, description = "get todo", body = [TodoResponse]),
        (status = 404, description = "todo not found", body = [ErrorResponse]),
        (status = 500, description = "database error", body = [ErrorResponse]),
    ),
    params(
        ("id" = u64, Path, description = "todo database id to get todo for"),
    )
)]
pub async fn get_todo_by_id(
    State(state): State<AppState>,
    WithRejection(Path(id), _): WithRejection<Path<i64>, ServiceError>,
) -> AppResult<TodoResponse> {
    let res = TodosEntity::find_by_id(id).one(&*state.database).await?;

    match res {
        Some(todo) => Ok(todo.into()),
        None => Err(ServiceError::TodoNotFoundError(id)),
    }
}

#[utoipa::path(
    put,
    request_body = UpdateTodoRequest,
    path = "/api/v1/todos/{id}",
    responses(
        (status = 200, description = "update todo", body = [TodoResponse]),
        (status = 201, description = "create todo", body = [ErrorResponse]),
        (status = 400, description = "invalid request", body = [ErrorResponse]),
        (status = 422, description = "lack of necessary fields", body = [ErrorResponse]),
        (status = 500, description = "database error", body = [ErrorResponse]),
    ),
    params(
        ("id" = u64, Path, description = "todo database id to get todo for"),
    )
)]
pub async fn put_todo_by_id(
    State(state): State<AppState>,
    WithRejection(Path(id), _): WithRejection<Path<i64>, ServiceError>,
    WithRejection(Json(payload), _): WithRejection<Json<UpdateTodoRequest>, ServiceError>,
) -> AppResult<(StatusCode, TodoResponse)> {
    payload.validate(&())?;

    let res = TodosEntity::find_by_id(id).one(&*state.database).await?;
    let mut todo: TodosActiveModel = match res {
        Some(todo) => todo.into(),
        None => {
            let payload: NewTodoRequest = payload.into();
            payload.validate(&())?;

            let todo = TodosActiveModel {
                id: ActiveValue::set(id),
                body: ActiveValue::set(payload.body),
                complated: ActiveValue::set(payload.complated.unwrap_or_default()),
                ..Default::default()
            };

            let res = todo.insert(&*state.database).await?;

            return Ok((StatusCode::CREATED, res.into()));
        }
    };

    if let Some(body) = payload.body {
        todo.body = ActiveValue::set(body);
    }
    if let Some(complated) = payload.complated {
        todo.complated = ActiveValue::set(complated);
    }
    todo.updated_at = ActiveValue::set(Utc::now().naive_utc());

    let res = todo.update(&*state.database).await?;

    Ok((StatusCode::OK, res.into()))
}

#[utoipa::path(
    delete,
    path = "/api/v1/todos/{id}",
    responses(
        (status = 200, description = "delete todo"),
        (status = 204, description = "todo not found"),
        (status = 500, description = "database error", body = [ErrorResponse]),
    ),
    params(
        ("id" = u64, Path, description = "todo database id to get todo for"),
    )
)]
pub async fn delete_todo_by_id(
    State(state): State<AppState>,
    WithRejection(Path(id), _): WithRejection<Path<i64>, ServiceError>,
) -> AppResult<StatusCode> {
    let res = TodosEntity::delete_by_id(id).exec(&*state.database).await?;

    if res.rows_affected == 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::OK)
    }
}
