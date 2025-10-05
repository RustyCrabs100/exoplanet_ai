mod ai;
#[path = "main-sk.rs"]
mod main_sk;
mod ui;

fn main() {
    ui::call_ui();
    ai::call_ai();
    main_sk::main_sk();
}
