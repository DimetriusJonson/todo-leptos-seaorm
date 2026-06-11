pub struct UserRoutes;

impl UserRoutes {
    pub fn base_segment() -> &'static str {
        "user"
    }

    pub fn base_url() -> &'static str {
        "/user"
    }

    pub fn create_segment() -> &'static str {
        "create"
    }

    pub fn create_url() -> &'static str {
        "/user/create"
    }

    pub fn login_segment() -> &'static str {
        "login"
    }

    pub fn login_url() -> &'static str {
        "/user/login"
    }

    pub fn login_url_with_params(def_user_name: &str, redirect_to: &str) -> String {
        format!("/user/login?redirectTo={}&defUserName={}", redirect_to, def_user_name)
    }

    pub fn label() -> &'static str {
        "User"
    }
}
