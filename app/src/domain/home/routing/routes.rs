pub struct HomeRoutes;

impl HomeRoutes {
    pub fn base_segment() -> &'static str {
        ""
    }

    pub fn base_url() -> &'static str {
        "/"
    }

    pub fn label() -> &'static str {
        "TODO"
    }
    
    pub fn base_url_with_params(auth: i32) -> String {
        format!("/?auth={}", auth)
    }
}
