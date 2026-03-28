use std::fs;

struct DialogueSystem{
    path: str,
    vm: InkVM,
}

impl DialogueSystem{
    pub fn new(path: &str) -> Result<Self, String>{
        let json_content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read Ink JSON: {}", e))?;


        let vm   = InkVM::new(&json_content).expect("Failed to create Ink VM");

        Ok( DialogueSystem{
            path: path.to_string(),
            vm,
        }
        );


    }

    pub fn start_story(&mut self)-> Result<String,int>{

        loop {

            match vm.continue_story(){
                Ok(outputs) => {
                    if outputs.is_empty() && vm.get_choices().is_empty() && vm.get_call_stack_depth() == 0 {
                        break;
                    }
                    for output in outputs {
                        match output {
                            InkOutput::Text(text) => {
                                println!("TEXT: {}", text);
                            }
                            InkOutput::Tag(tag) => {
                                println!("TAG: {}", tag);
                            }
                            InkOutput::Choice(choice) => {
                                println!("CHOICE {}: {}", choice.index, choice.text);
                            }
                        }
                    }
                }

            }

        }




    }
}