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
