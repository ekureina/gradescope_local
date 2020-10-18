/*
 * Copyright 2020 Claire Moore
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/
use std::fs;
use iced::{button, Button, Column, Element, Row, Sandbox, scrollable, Scrollable, Settings, Text, text_input, TextInput};

mod gradescope;

#[derive(Debug, Clone)]
enum GSMessage {
    RetrieveResults,
    PathChanged(String)
}

#[derive(Default)]
struct GSGui {
    // The result pulled from 
    grader_result: Option<gradescope::GraderResult>,
    // Where to grab the results
    retrieve_path: String,

    // The state of the GUI
    retrieve_button: button::State,
    retrieve_path_state: text_input::State,
    scrollable_state: scrollable::State,
}

impl Sandbox for GSGui {
    type Message = GSMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Gradescope Local")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Self::Message::RetrieveResults => {
                self.grader_result = match fs::read(&self.retrieve_path) {
                    Ok(vec) => {
                        serde_json::from_slice(vec.as_slice()).ok()
                    }
                    Err(_) => {
                        None
                    }
                }
            }
            Self::Message::PathChanged(path) => {
                self.retrieve_path = path;
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let results_column: Column<'_, Self::Message> = self.grader_result
            .as_ref()
            .map(display_results)
            .unwrap_or(Column::new().push(Text::new(String::from("No Results Loaded"))));
        Column::new()
        .push(
            Text::new("Gradescope Local is under Construction!")
        )
        .push(
            Row::new()
            .push(
                TextInput::new(
                    &mut self.retrieve_path_state,
                    "Enter the path to Gradescope's output",
                    &self.retrieve_path,
                    Self::Message::PathChanged
                )
                .on_submit(Self::Message::RetrieveResults)
            )
            .push(
                Button::new(
                    &mut self.retrieve_button,
                    Text::new("Retrieve Result")
                )
                .on_press(Self::Message::RetrieveResults)
            )
        )
        .push(
            Scrollable::new(&mut self.scrollable_state)
            .push(
                results_column
            )
        )
        .into()
    }
}

fn display_results<'a>(result: &gradescope::GraderResult) -> Column<'a, GSMessage> {
    let score = result.score
        .unwrap_or_else(
            || result.tests
                .as_ref()
                .unwrap()
                .iter()
                .fold(
                    0.0,
                    |full_score, test_score| full_score + test_score.score.unwrap()
                )
        );
    let test_output = match result.tests.as_ref() {
        Some(tests) => { 
            tests.iter().fold(Column::new().padding(15), |column, test| {
                let header = test.number.as_ref().unwrap().to_owned() + ") " + test.name.as_ref().unwrap_or(&String::from("")) + "(" +
                    &test.score.as_ref().unwrap().to_string() + " / " + &test.max_score.as_ref().unwrap().to_string() + ")";
                column
                .push(
                    Column::new()
                    .push(
                        Text::new(header)
                    )
                    .push(Text::new(test.output.as_ref().unwrap()))
                )
            })
        }
        None => {
            Column::new()
            .push(Text::new(String::from("No Tests")))
        }
    };
    Column::new().padding(15)
    .push(
        Row::new()
        .push(
            Text::new(String::from("Score: "))
        )
        .push(
            Text::new(score.to_string())
        )
    )
    .push(
        test_output
    )
}

fn main() -> std::result::Result<(), iced::Error> {
    GSGui::run(Settings::default())
}
