use serde::{Deserialize, Serialize};
use crate::{OS, TestCase};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) titleId: String,
    pub(crate) img: String,
    pub(crate) tests: Vec<Test>,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    pub(crate) tester: Option<String>,
    pub(crate) rating: i8,
    pub(crate) details: Option<String>,
    pub(crate) cpu: String,
    pub(crate) gpu: String,
    pub(crate) version: String,
    pub(crate) os: String,
    pub(crate) from_yuzu: bool,
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
            _ => unreachable!()
        },
        details: None,
        cpu: case.cpu.clone(),
        gpu: case.gpu.clone(),
        version: case.version.clone(),
        os: match case.os {
            OS::Windows => "Windows",
            OS::Linux => "Linux"
        }.to_string(),
        from_yuzu: true,
    }
}