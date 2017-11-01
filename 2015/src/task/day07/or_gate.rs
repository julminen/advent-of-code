use task::day07::Input;
use task::day07::Gate;
use task::day07::Memory;

// x OR y -> z
pub struct OrGate {
    input: (Input, Input),
    output: String,
}

impl OrGate {
    pub fn new(decl: &Vec<&str>) -> OrGate {
        let input = (match decl[0].parse() {
                         Ok(x) => Input::Direct(x),
                         Err(_) => Input::Indirect(decl[0].to_string()),
                     },
                     match decl[2].parse() {
                         Ok(x) => Input::Direct(x),
                         Err(_) => Input::Indirect(decl[2].to_string()),
                     });
        OrGate {
            input: input,
            output: decl[4].to_string(),
        }
    }
}
impl Gate for OrGate {
    fn operate(&self, mem: &mut Memory) -> bool {
        let output_key = self.output.clone();
        let input = (mem.get(&self.input.0), mem.get(&self.input.1));
        if input.0 != None && input.1 != None {
            mem.set(output_key, input.0.unwrap() | input.1.unwrap());
            true
        } else {
            false
        }
    }
    fn get_output_name(&self) -> &str {
        self.output.as_str()
    }
    fn get_input_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        match &self.input.0 {
            &Input::Direct(_) => {}
            &Input::Indirect(ref x) => names.push(x.clone()),
        };
        match &self.input.1 {
            &Input::Direct(_) => {}
            &Input::Indirect(ref x) => names.push(x.clone()),
        };

        names
    }
    fn to_string(&self) -> String {
        format!("{:?} OR {:?} -> {:?}",
                self.input.0,
                self.input.1,
                self.output)
    }
}
