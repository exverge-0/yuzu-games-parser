/*
MIT License

Copyright (c) 2024 Exverge (exverge@exverge.xyz)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */
use crate::{TestCase, OS};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub(crate) name: String,
    pub(crate) description: String,
    #[serde(rename = "titleId")]
    pub(crate) title_id: String,
    pub(crate) img: String,
    pub(crate) tests: Vec<Test>,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    pub tester: Option<String>,
    pub rating: i8,
    pub details: Option<String>,
    pub test_date: String,
    pub cpu: String,
    pub gpu: String,
    pub version: String,
    pub os: String,
    pub from_yuzu: bool,
}

pub fn testcase_to_test(case: &TestCase) -> Test {
    Test {
        tester: case.author.clone(),
        rating: match case.compatibility {
            0 => 6,
            1 => 5,
            2 => 4,
            3 => 3,
            4 => 2,
            5 => 1,
            _ => unreachable!(),
        },
        details: None,
        test_date: case.date.clone(),
        cpu: case.cpu.clone(),
        gpu: case.gpu.clone(),
        version: case.version.clone(),
        os: match case.os {
            OS::Windows => "Windows",
            OS::Linux => "Linux",
        }
        .to_string(),
        from_yuzu: true,
    }
}
