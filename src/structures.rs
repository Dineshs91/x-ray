use structures;


// project root
struct Root {
	name: &str,
	description: Option<&str>,
}

// python package, any directory with a __init__ file.
struct Package {
	name: &str,
	description: Option<&str>,
}

// python module, any python file.
struct Module {
	name: &str,
    description: Option<&str>,
}

// structure for a forming python function.
struct Function {
    name: &str,
    description: Option<&str>, // Option
    module: Module,
}

// structure for a forming python class.
struct Class {
	name: &str,
	description: Option<&str>,
	module: Module,
}