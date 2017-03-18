#[derive(Debug)]
#[derive(Deserialize)]
pub struct Config {
	pub root: Root,
}

// project root
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Root {
	pub name: String,
	pub description: Option<String>,
	pub modules: Vec<Module>,
}

// python module, any python file.
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Module {
	pub name: String,
    pub description: Option<String>,
	pub functions: Vec<Function>,
}

// structure for a forming python function.
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Function {
    pub name: String,
    pub description: Option<String>,
}