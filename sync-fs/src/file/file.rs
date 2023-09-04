#[derive(Debug, Clone)]
pub struct File {
   pub name: String,
}

impl File {
    pub fn New(input_path: String, output_path: String) -> File {
        File {
            name: String::from("f_name"),
        }
    }
}
