#[derive(Debug, Clone, PartialEq)]
pub enum Model {
    MitraLC,
    AskeyLC,
    MitraEconet,
    AskeyEconet,
    MitraWiFi6,
    AskeyWiFi6,
}

impl Model {
    pub(crate) fn from_sap_code(sap: &str) -> Option<Self> {
        match sap {
            "0192-0431-0" | "0192-0432-1" => Some(Model::MitraLC),
            "0192-0429-8" | "0192-0430-9" | "0192-0438-7" | "0192-0446-6" => Some(Model::AskeyLC),
            "0192-0452-2" | "0192-0453-3" | "0192-0476-0" | "0192-0477-0" => {
                Some(Model::MitraEconet)
            }
            "0192-0450-0" | "0192-0458-8" | "0192-0475-0" => Some(Model::AskeyEconet),
            "0192-0483-0" => Some(Model::MitraWiFi6),
            "0192-0484-0" => Some(Model::AskeyWiFi6),
            _ => None,
        }
    }
}
