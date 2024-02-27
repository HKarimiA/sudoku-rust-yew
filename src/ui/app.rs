use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{html, Component, Html, InputEvent};

use crate::models;

pub struct App {
    sudoku: models::sudoku::Sudoku,
    visible_incorrect_fields: Vec<(usize, usize)>,
}

pub enum Msg {
    OnInput(usize, usize, InputEvent),
    OnNew,
    OnNextStep,
    OnCheck,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self {
            sudoku: models::sudoku::Sudoku::new(),
            visible_incorrect_fields: vec![],
        }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> Html {
        html! {
          <html lang="en">
            <head>
              <title>{"Sudoku Rust-Yew"}</title>
              <style>{include_str!("../../sudoku-style.css")}</style>
            </head>
            <body>
              <div class="container">
                {Self::get_sudoku_table(&self, ctx)}
                <div class="button-container">
                  <button onclick={ctx.link().callback(|_| Msg::OnNew) } >{"New"}</button>
                  <button onclick={ctx.link().callback(|_| Msg::OnNextStep) } >{"Next step"}</button>
                  <button onclick={ctx.link().callback(|_| Msg::OnCheck) } >{"Check"}</button>
                </div>
              </div>
            </body>
          </html>
        }
    }

    fn update(&mut self, _ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnInput(row, col, e) => {
                self.update_input(row, col, e);
                true
            }
            Msg::OnNew => {
                self.sudoku = models::sudoku::Sudoku::new();
                self.visible_incorrect_fields.clear();
                true
            }
            Msg::OnNextStep => {
                if let Some((row, col, value)) = self.sudoku.next_step() {
                    self.sudoku.user_input(row, col, value);
                    return true;
                }
                let message = if self.sudoku.has_won() {
                    "Congratulations! You have won!"
                } else {
                    "Next step not found! Check if all your numbers are correct."
                };
                self.alert(message);
                false
            }
            Msg::OnCheck => {
                self.visible_incorrect_fields = self.sudoku.incorrect_fields();
                true
            }
        }
    }
}

impl App {
    fn get_sudoku_table(&self, ctx: &yew::prelude::Context<Self>) -> Html {
        html! {
            <table id="sudokuTable">
                { for (0..9).map(|row| self.get_sudoku_row(row, ctx)) }
            </table>
        }
    }

    fn get_sudoku_row(&self, row: usize, ctx: &yew::prelude::Context<Self>) -> Html {
        html! {
            <tr>
                { for (0..9).map(|col| self.get_sudoku_cell(row, col, ctx)) }
            </tr>
        }
    }

    fn get_sudoku_cell(&self, row: usize, col: usize, ctx: &yew::prelude::Context<Self>) -> Html {
        html! {
            <td class={self.get_field_class(row, col)} id={format!("cell-{}-{}", row, col)}>
                { self.get_sudoku_content(row, col, ctx) }
            </td>
        }
    }

    fn get_sudoku_content(
        &self,
        row: usize,
        col: usize,
        ctx: &yew::prelude::Context<Self>,
    ) -> Html {
        if self.sudoku.puzzle_sudoku[row][col] != 0 {
            html! {
                { self.sudoku.display_sudoku[row][col] }
            }
        } else {
            html! {
                <input
                    type="number"
                    min="1"
                    max="9"
                    id={format!("field-{}-{}", row, col)}
                    value={if self.sudoku.display_sudoku[row][col] == 0 {String::from("")} else {self.sudoku.display_sudoku[row][col].to_string()}}
                    oninput={ ctx.link().callback(move |e| Msg::OnInput(row, col, e))}
                />
            }
        }
    }

    fn get_field_class(&self, row: usize, col: usize) -> String {
        let mut class = String::from("cell");
        if self.sudoku.puzzle_sudoku[row][col] != 0 {
            class.push_str(" read-only");
        }
        if self.visible_incorrect_fields.contains(&(row, col)) {
            class.push_str(" incorrect");
        }
        if (col + 1) % 3 == 0 && col != 8 {
            class.push_str(" border-right");
        }
        if (row + 1) % 3 == 0 && row != 8 {
            class.push_str(" border-bottom");
        }
        class
    }

    fn update_input(&mut self, row: usize, col: usize, e: InputEvent) {
        let input = e
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            .map(|t| t.value())
            .unwrap_or_default();
        if input.is_empty() {
            self.sudoku.user_input(row, col, 0);
        } else if input.len() == 1 && input.chars().all(char::is_numeric) {
            let value = input.parse::<u8>().unwrap_or(0);
            self.sudoku.user_input(row, col, value);
        }
        let index = self
            .visible_incorrect_fields
            .iter()
            .position(|&x| x == (row, col));
        if let Some(index) = index {
            self.visible_incorrect_fields.remove(index);
        }
        if self.sudoku.has_won() {
            self.alert("Congratulations! You have won!");
        }
    }

    fn alert(&self, message: &str) {
        web_sys::window()
            .unwrap()
            .alert_with_message(message)
            .unwrap();
    }
}
