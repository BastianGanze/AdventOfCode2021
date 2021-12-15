use std::collections::HashMap;

pub type Polymer = Vec<char>;
pub type PolyConstructionInstructions = HashMap<(char, char), (char, char, char)>;
pub type Instructions = HashMap<(char, char), Instruction>;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub count_change: HashMap<(char, char), i8>,
    pub base_count_change: HashMap<char, i8>,
}

impl Instruction {
    pub fn to_string(&self) -> String {
        let mut str = String::from("");

        str.push_str("  { ");
        for (instruction, num) in self.count_change.iter() {
            str.push_str(format!("{}{} -> {}; ", instruction.0, instruction.1, num).as_str());
        }
        str.pop();
        str.push_str(" }  { ");
        for (base, num) in self.base_count_change.iter() {
            str.push_str(format!("{} -> {}; ", base, num).as_str());
        }
        str.pop();
        str.push_str(" }");
        str
    }
}

pub type InstructionFrequencyMap = HashMap<(char, char), i64>;
pub type BaseFrequencyMap = HashMap<char, i64>;

pub type ParseOutput = (Instructions, InstructionFrequencyMap, BaseFrequencyMap);

pub fn read_main() -> String {
    read_file("src/14.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let mut poly_construction_instructions: PolyConstructionInstructions = HashMap::new();
    let mut actual_instructions: Instructions = HashMap::new();
    let mut instruction_frequency: InstructionFrequencyMap = HashMap::new();
    let mut base_frequency: BaseFrequencyMap = HashMap::new();

    let (start_poly, instruction_lines) = file.split_once("\n\n").unwrap();

    for instruction_line in instruction_lines.split("\n") {
        let (from, to_str) = instruction_line.split_once(" -> ").unwrap();
        let to = to_str.chars().nth(0).unwrap();
        let mut from_as_chars = from.chars();
        let (from_0, from_1) = (from_as_chars.nth(0).unwrap(), from_as_chars.nth(0).unwrap());

        instruction_frequency.insert((from_0, from_1).clone(), 0);
        base_frequency.insert(from_0, 0);
        base_frequency.insert(from_1, 0);
        base_frequency.insert(to, 0);

        poly_construction_instructions.insert((from_0, from_1), (from_0, to, from_1));
    }

    let mut last_char_option: Option<char> = None;
    for current_char in start_poly.chars().into_iter() {
        if let Some(last_char) = last_char_option {
            if let Some(instruction_freq) =
                instruction_frequency.get_mut(&(last_char, current_char))
            {
                *instruction_freq += 1;
            }
        }
        *base_frequency.get_mut(&current_char).unwrap() += 1;
        last_char_option = Some(current_char);
    }

    for (start, result) in poly_construction_instructions.iter() {
        actual_instructions.insert(
            start.clone(),
            Instruction {
                base_count_change: HashMap::new(),
                count_change: HashMap::new(),
            },
        );

        let instruction = actual_instructions.get_mut(&start).unwrap();

        instruction.count_change.insert(start.clone(), -1); // The start one is no longer in the polymer

        let first_pair = (result.0, result.1);
        let second_pair = (result.1, result.2);

        ignore_result(instruction.count_change.try_insert(first_pair.clone(), 0));
        ignore_result(instruction.count_change.try_insert(second_pair.clone(), 0));

        if poly_construction_instructions.get(&first_pair).is_some() {
            *instruction.count_change.get_mut(&first_pair).unwrap() += 1;
        }
        if poly_construction_instructions.get(&first_pair).is_some() {
            *instruction.count_change.get_mut(&second_pair).unwrap() += 1;
        }

        ignore_result(instruction.base_count_change.try_insert(result.1, 0));

        *instruction.base_count_change.get_mut(&result.1).unwrap() += 1;
    }

    (actual_instructions, instruction_frequency, base_frequency)
}

fn ignore_result<T>(_: T) {}
