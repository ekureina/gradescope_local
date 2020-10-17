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
use iced::{Element, Sandbox, Settings, Text};

mod gradescope;

struct GSGui;

impl Sandbox for GSGui {
    type Message = ();

    fn new() -> GSGui {
        GSGui
    }

    fn title(&self) -> String {
        String::from("Gradescope Local")
    }

    fn update(&mut self, _message: Self::Message) {
        // Boilerplate, no updates currently
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Gradescope Local is under Construction!").into()
    }
}

fn main() {
    GSGui::run(Settings::default());
} 
