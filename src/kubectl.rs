use std::process;
use termion::style;

pub fn set_context_namespace(context: &str, namespace: &str) -> Result<(), &'static str> { // TODO: refactor errors to remove str types
    if !namespace.is_empty() {
        if let Err(e) = update_namespace(context, namespace){
            return Err(e)
        }
    }

    let args = vec!["config", "use-context", context];
    match run(args) {
        Ok(_) => {
                print!("Working in the {} {} {} namespace of the {} {} {} context.\r\n",
                       style::Invert, namespace, style::Reset, style::Invert, context, style::Reset);
        },
        Err(e) => return Err(e),
    };

    Ok(())
}

fn update_namespace(context: &str, namespace: &str) -> Result<(), &'static str> {
    let args = vec!["config", "set-context", context, "--namespace", namespace];

    match run(args) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn get_contexts() -> Result<Vec<String>, &'static str> {
    get_vec(["config","get-contexts","-o=name"].to_vec())
}

pub fn get_namespaces() -> Result<Vec<String>, &'static str> {
    get_vec(["get","namespaces","--no-headers","-o=custom-columns=:.metadata.name"].to_vec())
}

pub fn get_namespace_for_context(context: &str) -> Result<String, &'static str> {
    let jsonpath = format!("-o=jsonpath=\"{{.contexts[?(@.name==\"{}\")].context.namespace}}\"", context);

    get_string(["config","view",&jsonpath].to_vec())
}

pub fn get_context() -> Result<String, &'static str> {
    get_string(["config","current-context"].to_vec())
}

fn get_string(args: Vec<&str>) -> Result<String, &'static str> {
    let output = match run(args) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    Ok(String::from_utf8(output.stdout).unwrap()
       .trim()
       .trim_matches(|c| c == '"' || c == '\'')
       .to_string())
}

fn get_vec(args: Vec<&str>) -> Result<Vec<String>, &'static str> {
    let output = match run(args) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let list: Vec<String> = String::from_utf8(output.stdout).unwrap() // TODO: handle unwrap?
        .trim().split('\n').map(|s| s.to_string()).collect();

    Ok(list)
}

fn run(args: Vec<&str>) -> Result<process::Output, &'static str> {
    let output = match process::Command::new("kubectl").args(args).output() {
        Ok(o) => o,
        Err(_) => return Err("could not find kubectl executable in path"),
    };

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("could not invoke kubectl");
    }

    Ok(output)
}
