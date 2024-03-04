
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    }
}

impl Command {  //we can add methods and assoicated functions to enums using impl blocks
    fn serialize(&self) -> String{
        String::from("JSON String")
    }
}

fn main() {
                                  
    let cmd1 = Command::Undo; // Enums can represent variants
    let cmd2 = Command::AddText(String::from("test"));
    let cmd3 = Command::MoveCursor(22, 0);
    let cmd4 = Command::Replace {
        from: String::from("a"), // And these variants can contain data
        to: String::from("b"),
    };

    let json_string = cmd.serialize();
}
