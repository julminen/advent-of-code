use task::day07::Input;
use task::day07::Gate;
use task::day07::Memory;

// NOT a -> b
pub struct NotGate {
    input: Input,
    output: String,
}

impl NotGate {
    pub fn new(decl: &Vec<&str>) -> NotGate {
        let ip = match decl[1].parse() {
            Ok(x) => Input::Direct(x),
            Err(_) => Input::Indirect(decl[1].to_string()),
        };
        NotGate {
            input: ip,
            output: decl[3].to_string(),
        }
    }
}

impl Gate for NotGate {
    fn operate(&self, mem: &mut Memory) -> bool {
        let output_key = self.output.clone();
        match mem.get(&self.input) {
            Some(x) => {
                mem.set(output_key, !x);
                true
            }
            None => false,
        }
    }
    fn get_output_name(&self) -> &str {
        self.output.as_str()
    }
    fn get_input_names(&self) -> Vec<String> {
        match &self.input {
            &Input::Direct(_) => vec![],
            &Input::Indirect(ref x) => vec![x.clone()],
        }
    }
    fn to_string(&self) -> String {
        format!("NOT {:?} -> {:?}", self.input, self.output)
    }
}
