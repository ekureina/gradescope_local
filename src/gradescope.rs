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

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResult {
    score: Option<f32>,
    max_score: Option<f32>,
    number: Option<String>,
    output: Option<String>,
    tags: Option<Vec<String>>,
    visibility: Option<Visibility>,
    extra_data: Option<Value>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum Visibility {
    Hidden,
    AfterPublished,
    AfterDueDate,
    Visible
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraderResult {
    score: Option<f32>,
    execution_time: Option<u32>,
    output: Option<String>,
    visibility: Option<Visibility>,
    stdout_visibility: Option<Visibility>,
    extra_data: Option<Value>,
    tests: Option<Vec<TestResult>>
    // No leaderboard Support
}
