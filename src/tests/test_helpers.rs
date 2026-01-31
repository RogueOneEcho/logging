use colored::control;
use std::env;

pub fn force_truecolor() {
    env::set_var("COLORTERM", "truecolor");
    control::set_override(true);
}
