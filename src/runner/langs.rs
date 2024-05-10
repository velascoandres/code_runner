pub struct BuildArgs {
    pub id: String,
    pub path: String,
    pub params: String,
}

pub trait LangCommand {
    fn make_build_command(args: BuildArgs) -> (String, Vec<String>);
}

#[derive(Default)]
pub struct JavascriptCommand;
#[derive(Default)]

pub struct RustCommand;
#[derive(Default)]

pub struct PythonCommand;


impl LangCommand for JavascriptCommand {
    fn make_build_command(args: BuildArgs) -> (String, Vec<String>) {
        let id = args.id;
        let path = args.path;
        let params = args.params;

        let main_js_file = format!("node main.js {params}");
        let code_path = format!("{path}:/code");

        let args = vec![
            "run",
            "--rm",
            "-m",
            "64M",
            "--memory-swap",
            "64M",
            "--name",
            &id,
            "-v",
            &code_path,
            "-w",
            "/code",
            "node:current-alpine3.15",
            "/bin/sh",
            "-c",
            &main_js_file,
        ]
        .into_iter()
        .map(|arg| arg.to_string())
        .collect();

        ("docker".to_string(), args)
    }
}

impl LangCommand for PythonCommand {
    fn make_build_command(args: BuildArgs) -> (String, Vec<String>) {
        let id = args.id;
        let path = args.path;
        let params = args.params;

        let main_js_file = format!("\"node main.js '{params}'\"");
        let code_path = format!("{path}:/code");

        let args = vec![
            "run",
            "--rm",
            "-m",
            "64M",
            "--memory-swap",
            "64M",
            "--name",
            &id,
            "-v",
            &code_path,
            "-w",
            "/code",
            "node:current-alpine3.15",
            "/bin/sh",
            "-c",
            &main_js_file,
        ]
        .into_iter()
        .map(|arg| arg.to_string())
        .collect();

        ("docker".to_string(), args)
    }
}

impl LangCommand for RustCommand {
    fn make_build_command(args: BuildArgs) -> (String, Vec<String>) {
        let id = args.id;
        let path = args.path;
        let params = args.params;

        let main_js_file = format!("\"node main.js '{params}'\"");
        let code_path = format!("{path}:/code");

        let args = vec![
            "run",
            "--rm",
            "-m",
            "64M",
            "--memory-swap",
            "64M",
            "--name",
            &id,
            "-v",
            &code_path,
            "-w",
            "/code",
            "node:current-alpine3.15",
            "/bin/sh",
            "-c",
            &main_js_file,
        ]
        .into_iter()
        .map(|arg| arg.to_string())
        .collect();

        ("docker".to_string(), args)
    }
}
