use std::collections::BTreeMap;

use rocket_dyn_templates::Template;

const DASHBOARD_TEMPLATE_NAME: &str = "dashboard";

#[get("/")]
pub fn get_dashboard() -> Template {
    let data: BTreeMap<&str, String> = BTreeMap::new();
    Template::render(DASHBOARD_TEMPLATE_NAME, data)
}
