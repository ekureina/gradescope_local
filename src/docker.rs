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

use std::path::Path;
use std::fs;
use std::process::Output;
use iced::{button, scrollable, text_input, Button, Column, Command, Element, Row, Scrollable, Text, TextInput};
use crate::{gradescope, results};

#[derive(Debug, Clone)]
pub enum Message {
    RunDocker,
    SourcePathChanged(String),
    SubmissionPathChanged(String),
    RanDocker(Option<Output>),
    ResultMessage(results::Message)
}

#[derive(Default)]
pub struct RunGradescope {
    source_path: String,
    submission_path: String,
    grader_result: Option<gradescope::GraderResult>,

    run_docker: button::State,
    source_path_state: text_input::State,
    submission_path_state: text_input::State,
    scrollable_state: scrollable::State
}

impl RunGradescope {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RunDocker => {
                Command::perform(run_docker(self.source_path.clone(), self.submission_path.clone()), Message::RanDocker)
            },
            Message::SourcePathChanged(new_path) => {
                self.source_path = new_path;
                Command::none()
            },
            Message::SubmissionPathChanged(new_path) => {
                self.submission_path = new_path;
                Command::none()
            },
            Message::RanDocker(docker_output) => {
                match docker_output {
                    Some(_output) => {
                        self.grader_result = match fs::read("results.json") {
                            Ok(vec) => {
                                serde_json::from_slice(vec.as_slice()).ok()
                            }
                            Err(_) => {
                                None
                            }
                        };
                    }
                    None => {
                        eprintln!("No Output!");
                    }
                };
                fs::remove_file("results.json").unwrap();
                Command::none()
            },
            Message::ResultMessage(_) => {
                Command::none() // No message from the results display we care about
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let result_display: Element<Message> = match self.grader_result.as_ref() {
            Some(result) => {
                Scrollable::new(&mut self.scrollable_state)
                    .push(
                        results::Visualizer::view_result(&result).map(Message::ResultMessage)
                    )
                    .into()
            },
            None => { Text::new(String::from("No Results")).into() }
        };
        Column::new()
            .push(
                TextInput::new(
                    &mut self.submission_path_state,
                    "Enter the path to the submission source",
                    &self.submission_path,
                    Message::SubmissionPathChanged
                )
                .on_submit(Message::RunDocker)
            )
            .push(
                Row::new()
                    .push(
                        TextInput::new(
                            &mut self.source_path_state,
                            "Enter the path to the autograder source",
                            &self.source_path,
                            Message::SourcePathChanged
                        )
                        .on_submit(Message::RunDocker)
                    )
                    .push(
                        Button::new(
                            &mut self.run_docker,
                            Text::new(String::from("Run Locally"))
                        )
                        .on_press(Message::RunDocker)
                    )
            )
            .push(result_display)
            .into()
    }
}

async fn run_docker(source_path: String, submission_path: String) -> Option<Output> {
    let source = Path::new(&source_path).canonicalize().unwrap();
    let docker_workspace = source.parent().unwrap();
    let source_zip = source.file_name().unwrap();
    let submission = move_to_workspace(docker_workspace, &Path::new(&submission_path).canonicalize().unwrap());

    run_docker_base(docker_workspace, source_zip.to_str().unwrap(), &submission)
}

fn move_to_workspace(docker_workspace: &Path, source: &Path) -> String {
    match source.strip_prefix(docker_workspace) {
        Ok(source_path) => String::from(source_path.to_str().unwrap()),
        Err(_) => {
            let source_path = docker_workspace.join(source.file_name().unwrap());
            #[cfg(target_family = "unix")]
            std::os::unix::fs::symlink(source, source_path).unwrap();
            #[cfg(target_family = "windows")]
            {
                if source.is_dir() {
                    std::os::windows::fs::symlink_dir(source, source_path);
                } else {
                    std::os::windows::fs::symlink_file(source, source_path);
                }
            }
            String::from(source.file_name().unwrap().to_str().unwrap())
        }
    }
}

fn run_docker_base(docker_workspace: &Path, source_zip: &str, submission_path: &str) -> Option<Output> {
    let docker_commands = format!(include_str!("../rsrc/dockerfile.gradescope.in"), source_zip, submission_path);
    fs::write(docker_workspace.join("Dockerfile"), docker_commands).unwrap();
    let shell_script = format!(include_str!("../rsrc/run_local.sh"), docker_workspace.to_str().unwrap());
    std::process::Command::new("sh")
        .arg("-c")
        .arg(shell_script)
        .output().ok()
}
