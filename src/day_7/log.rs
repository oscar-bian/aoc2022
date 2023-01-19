pub fn is_command(line: &Vec<String>) -> bool {
    line[0] == "$"
}

pub fn is_move_command(line: &Vec<String>) -> bool {
    line[1] == "cd"
}

pub fn is_move_up_command(line: &Vec<String>) -> bool {
    line[2] == ".."
}

pub fn is_list_command(line: &Vec<String>) -> bool {
    line[1] == "ls"
}
