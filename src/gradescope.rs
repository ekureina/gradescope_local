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

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct TestResult {
    pub score: Option<f32>,
    pub max_score: Option<f32>,
    pub name: Option<String>,
    pub number: Option<String>,
    pub output: Option<String>,
    pub tags: Option<Vec<String>>,
    pub visibility: Option<Visibility>,
    pub extra_data: Option<Value>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Hidden,
    AfterPublished,
    AfterDueDate,
    Visible
}

#[derive(Deserialize, Debug)]
pub struct GraderResult {
    pub score: Option<f32>,
    pub execution_time: Option<u32>,
    pub output: Option<String>,
    pub visibility: Option<Visibility>,
    pub stdout_visibility: Option<Visibility>,
    pub extra_data: Option<Value>,
    pub tests: Option<Vec<TestResult>>
    // No leaderboard Support
}
