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

use iced::{button, text_input, Button, Column, Element, Row, Text, TextInput};

#[derive(Debug, Clone)]
pub enum Message {
    RunDocker,
    PathChanged(String)
}

const DOCKER_TEMPLATE: &str = include_str!("../rsrc/dockerfile.gradescope.in");

#[derive(Default)]
pub struct RunGradescope {
    source_path: String,

    run_docker: button::State,
    source_path_state: text_input::State,
}

impl RunGradescope {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::RunDocker => {}, // To Implement
            Message::PathChanged(new_path) => {
                self.source_path = new_path;
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Row::new()
                    .push(
                        TextInput::new(
                            &mut self.source_path_state,
                            "Enter the path to the autograder source",
                            &self.source_path,
                            Message::PathChanged
                        )
                    )
                    .push(
                        Button::new(
                            &mut self.run_docker,
                            Text::new(String::from("Run Locally"))
                        )
                        .on_press(Message::RunDocker)
                    )
            )
            .push(Text::new(String::from(DOCKER_TEMPLATE)))
        .into()
    }
}
