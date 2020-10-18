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
use iced::{button, scrollable, text_input, Button, Column, Element, Row, Scrollable, Text, TextInput};

use crate::gradescope;

#[derive(Debug, Clone)]
pub enum Message {
    RetrieveResults,
    PathChanged(String)
}

#[derive(Default)]
pub struct Visualizer {
    // The result pulled from 
    grader_result: Option<gradescope::GraderResult>,
    // Where to grab the results
    retrieve_path: String,

    // The state of the GUI
    retrieve_button: button::State,
    retrieve_path_state: text_input::State,
    scrollable_state: scrollable::State
}


impl Visualizer {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::RetrieveResults => {
                self.grader_result = match fs::read(&self.retrieve_path) {
                    Ok(vec) => {
                        serde_json::from_slice(vec.as_slice()).ok()
                    }
                    Err(_) => {
                        None
                    }
                }
            }
            Message::PathChanged(path) => {
                self.retrieve_path = path;
            }
        }
    }

    pub fn view_result(result: &gradescope::GraderResult) -> Element<Message> {
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
                tests.iter().fold(Column::new().spacing(15), |column, test| {
                    column.push(Self::view_test(test))
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
            .into()
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .spacing(10)
            .padding(10)
            .push(
                Row::new()
                    .push(
                        TextInput::new(
                            &mut self.retrieve_path_state,
                            "Enter the path to Gradescope's output",
                            &self.retrieve_path,
                            Message::PathChanged
                        )
                        .on_submit(Message::RetrieveResults)
                    )
                    .push(
                        Button::new(
                            &mut self.retrieve_button,
                            Text::new("Retrieve Result")
                        )
                        .on_press(Message::RetrieveResults)
                    )
            )
            .push(
                Scrollable::new(&mut self.scrollable_state)
                    .push(
                        self.grader_result
                            .as_ref()
                            .map(Self::view_result)
                            .unwrap_or(
                                Column::new()
                                    .push(Text::new(String::from("No Results Loaded")))
                                    .into()
                            )
                    )
            )
            .into()
    }

    fn view_test(test: &gradescope::TestResult) -> Element<Message> {
        Column::new().spacing(10)
            .push(Text::new(Self::create_test_header(test)))
            .push(Text::new(test.output.as_ref().unwrap()))
            .into()
    }

    fn create_test_header(test: &gradescope::TestResult) -> String {
        test.number.as_ref().unwrap().to_owned() + ") " + test.name.as_ref().unwrap_or(&String::from("")) + "(" +
            &test.score.as_ref().unwrap().to_string() + " / " + &test.max_score.as_ref().unwrap().to_string() + ")"
    }
}
