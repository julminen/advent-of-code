use task::day07::Input;
use task::day07::Gate;
use task::day07::Memory;

// a -> b
pub struct DirectGate {
    input: Input,
    output: String,
}

impl DirectGate {
    pub fn new(decl: &Vec<&str>) -> DirectGate {
        let ip = match decl[0].parse() {
            Ok(x) => Input::Direct(x),
            Err(_) => Input::Indirect(decl[0].to_string()),
        };
        DirectGate {
            input: ip,
            output: decl[2].to_string(),
        }
    }
}

impl Gate for DirectGate {
    fn operate(&self, mem: &mut Memory) -> bool {
        let output_key = self.output.clone();
        match mem.get(&self.input) {
            Some(x) => {
                mem.set(output_key, x);
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
        format!("{:?} -> {:?}", self.input, self.output)
    }
}
