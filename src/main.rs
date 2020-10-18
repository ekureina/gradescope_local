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
use iced::{button, Button, Column, Element, Row, Rule, Sandbox, Settings, Text};

mod gradescope;
mod results;

#[derive(Debug, Clone)]
enum Message {
    ChangeState(State),
    ResultMessage(results::Message)
}

#[derive(Debug, Clone, Copy)]
enum State {
    Home,
    Visualizer
}

impl Default for State {
    fn default() -> State {
        State::Home
    }
}

#[derive(Default)]
struct GSGui {
    state: State,
    visualizer: results::Visualizer,

    home_state: button::State,
    visualizer_state: button::State
}

impl Sandbox for GSGui {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        let tab = match self.state {
            State::Home => "Home",
            State::Visualizer => "Visualizer"
        };
        String::from("Gradescope Local: ") + tab
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ResultMessage(result_message) => {
                self.visualizer.update(result_message)
            }
            Message::ChangeState(new_state) => {
                self.state = new_state
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let tab = match self.state {
            State::Home => {
                Text::new(String::from("Welcome to Gradescope Local")).into()
            }
            State::Visualizer => {
                self.visualizer
                    .view()
                    .map(Message::ResultMessage)
            }
        };

        Column::new()
            .spacing(5)
            .push(
                Row::new()
                    .push(
                        Button::new(
                            &mut self.home_state,
                            Text::new(String::from("Home"))
                        )
                        .on_press(Message::ChangeState(State::Home))
                    )   
                    .push(
                        Button::new(
                            &mut self.visualizer_state,
                            Text::new(String::from("Visualizer"))
                        )
                        .on_press(Message::ChangeState(State::Visualizer))
                    )   
            )
            .push(Rule::horizontal(0))
            .push(tab)
            .into()
    }
}

fn main() -> std::result::Result<(), iced::Error> {
    GSGui::run(Settings::default())
}
