use super::config;
use super::kubectl;

pub fn run(config: &config::Opt) -> Result<(),&'static str> { // TODO: refactor errors to remove str types
    let contexts = kubectl::get_contexts()?;

    match &config.arg1 {
        Some(arg1) => match &config.arg2 {
            Some(arg2) => kubectl::set_context_namespace(arg1, arg2),
            None => {
                let current_context = kubectl::get_context()?;

                for c in contexts {
                   if &c == arg1 {
                       let current_namespace = kubectl::get_namespace_for_context(arg1)?;
                       return kubectl::set_context_namespace(arg1, &current_namespace)
                   }
                }

                let namespaces = kubectl::get_namespaces_for_context(&current_context)?;
                for n in namespaces {
                    if &n == arg1 {
                        return kubectl::set_context_namespace(&current_context, arg1)
                    }
                }

                Err("context (or namespace in the current context) does not exist")
            },
        },
        None => Ok(()),
    }
}
