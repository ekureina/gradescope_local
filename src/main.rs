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
use iced::{button, Button, Column, Element, Row, Sandbox, Settings, Text, text_input, TextInput};

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
    retrieve_path_state: text_input::State
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
                println!("{}", self.retrieve_path)
            }
            Self::Message::PathChanged(path) => {
                self.retrieve_path = path;
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
        .push(
            Text::new("Gradescope Local is under Construction!")
        )
        .push(
            Row::new()
            .push(
                TextInput::new(
                    &mut self.retrieve_path_state,
                    "Enterr the path to Gradescope's output",
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
        .into()
    }
}

fn main() -> std::result::Result<(), iced::Error> {
    GSGui::run(Settings::default())
}
