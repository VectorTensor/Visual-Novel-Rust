use std::fs;

fn main() {
    // Load the Ink JSON file
    let json_content = fs::read_to_string("assets/DialogueFlow/test.ink.json")
        .expect("Failed to read test.ink.json");
    
    // Create and initialize the VM
    let mut vm = visual_novel::ink_vm::InkVM::new(&json_content)
        .expect("Failed to create Ink VM");
    
    vm.initialize().expect("Failed to initialize VM");
    
    println!("=== Ink VM Test ===\n");
    
    // Continue the story until it ends or hits choices
    loop {
        match vm.continue_story() {
            Ok(outputs) => {
                if outputs.is_empty() && vm.get_choices().is_empty() && vm.get_call_stack_depth() == 0 {
                    break;
                }
                for output in outputs {
                    match output {
                        visual_novel::ink_vm::InkOutput::Text(text) => {
                            println!("TEXT: {}", text);
                        }
                        visual_novel::ink_vm::InkOutput::Tag(tag) => {
                            println!("TAG: {}", tag);
                        }
                        visual_novel::ink_vm::InkOutput::Choice(choice) => {
                            println!("CHOICE {}: {}", choice.index, choice.text);
                        }
                        visual_novel::ink_vm::InkOutput::End => {
                            println!("--- END ---");
                            return;
                        }
                    }
                }
                if !vm.get_choices().is_empty() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
    
    // Display available choices
    let choices = vm.get_choices();
    if !choices.is_empty() {
        println!("\nAvailable choices:");
        for choice in choices {
            println!("  [{}] {}", choice.index, choice.text);
        }
    }
}
