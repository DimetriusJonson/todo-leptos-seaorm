use app::app::{App, shell};
use app::common::DbPool;
use app::common::app_state::ssr::AppState;
use app::common::security_context::SecurityContext;
use app::domain::user::model::user::User;
use app::domain::user::user_services::ssr::is_valid_token;
use axum::body::Body as AxumBody;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Router, middleware};
use leptos::prelude::*;
use leptos_axum::{
    LeptosRoutes, generate_route_list, handle_server_fns_with_context,
    render_app_to_stream_with_context,
};
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub async fn build_app_router(conf_file: ConfFile, pool: DbPool) -> anyhow::Result<Router> {
    let leptos_options = conf_file.leptos_options;

    let routes = generate_route_list(|| view! { <App /> });

    let app_state = AppState { leptos_options: leptos_options.clone(), pool: pool.clone() };

    Ok(Router::new()
        .route("/api/{*fn_name}", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), check_auth_token))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(CompressionLayer::new().gzip(true))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state))
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[axum_macros::debug_handler]
pub async fn server_fn_handler(
    State(state): State<AppState>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(state.clone());
        },
        request,
    )
    .await
}

#[axum_macros::debug_handler]
pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let leptos_options = app_state.leptos_options.clone();

    let handler = render_app_to_stream_with_context(
        move || {
            provide_context(app_state.clone());
        },
        move || shell(leptos_options.clone()),
    );
    handler(req).await.into_response()
}

pub async fn check_auth_token(mut request: Request<AxumBody>, next: Next) -> Response {
    use axum_extra::extract::CookieJar;

    let jar = CookieJar::from_headers(request.headers());
    let token = jar.get("todo-token").map(|c| c.value().to_string()).unwrap_or_default();

    let claims = is_valid_token(&token).unwrap();
    let user;
    if let Some(claims) = claims {
        user = Some(User {
            id: Some(claims.user_id),
            username: Some(claims.user_name),
            token: Some(token.to_owned()),
            ..Default::default()
        });
    } else {
        user = None;
    }

    request.extensions_mut().insert(SecurityContext { user });

    next.run(request).await
}
