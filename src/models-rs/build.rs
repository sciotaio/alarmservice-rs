use std::{
    fs::{self, File},
    process::{exit, Command, Stdio},
};

// macro for printing warning message
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    p!("### Building models in crate '{name} {version}' ###");

    // prepare source paths for rs and ts models
    let generated_src_path_rs = "../openapi/.generated/rs";
    let generated_src_path_ts = "../openapi/.generated/ts";
    let openapi_spec_path = "../openapi/api.yml";

    // tell Cargo when the build script should be executed
    println!("cargo::rerun-if-changed={openapi_spec_path}");
    println!("cargo::rerun-if-changed=build.rs");

    // prepare destination paths for rs and ts models
    let generated_dest_path_rs = "src/models/";
    let generated_dest_path_ts = "../ts/models/";

    // prepare path for openapi-generator
    let openapi_generator_path = "../openapi/generator-cli/openapi-generator-cli-7.2.0.jar";

    // check if java is present
    let mut java_cmd = Command::new("java");
    let java_mut_cmd = java_cmd.arg("--version");
    run_cmd(
        java_mut_cmd,
        "java",
        "please make sure that java is installed on your system",
    );

    // create folders if not exists
    create_dir_if_missing(generated_src_path_rs);
    create_dir_if_missing(generated_src_path_ts);

    // clean up source folders for generated rs and ts models
    remove_dir_contents(generated_src_path_rs);
    remove_dir_contents(generated_src_path_ts);

    // generate openapi models for rs
    let openapi_rs_templates_path = "../openapi/templates/rust-and-tsify";
    let mut openapi_generate_rs_cmd = Command::new("java");
    let openapi_generate_rs_mut_cmd = openapi_generate_rs_cmd
        .arg("-jar")
        .arg(openapi_generator_path)
        .arg("generate")
        .args(["--package-name", "api-spec"])
        .args(["-g", "rust"])
        .args(["--model-package", "dtos"])
        .args(["--model-name-suffix", "dto"])
        .args(["-o", generated_src_path_rs])
        .args(["-i", openapi_spec_path])
        .args(["--template-dir", openapi_rs_templates_path]);
    run_cmd(
        openapi_generate_rs_mut_cmd,
        "openapi-generator rs",
        "failed to generate openapi rs models",
    );

    // generate openapi models for ts
    let openapi_ts_templates_path = "../openapi/templates/typescript-node";
    let mut openapi_generate_ts_cmd = Command::new("java");
    let openapi_generate_ts_mut_cmd = openapi_generate_ts_cmd
        .arg("-jar")
        .arg(openapi_generator_path)
        .arg("generate")
        .args(["--package-name", "api-spec"])
        .args(["-g", "typescript-node"])
        .args(["--global-property", "models"])
        .args(["--model-name-suffix", "dto"])
        .args(["-o", generated_src_path_ts])
        .args(["-i", openapi_spec_path])
        .args(["--type-mappings", "string+date-time=string"])
        .args(["--template-dir", openapi_ts_templates_path])
        .arg("--additional-properties=modelPropertyNaming=original");
    run_cmd(
        openapi_generate_ts_mut_cmd,
        "openapi-generator ts",
        "failed to generate openapi ts models",
    );

    // clean up destination folders for generated rs and ts models
    remove_dir_contents(generated_dest_path_rs);
    remove_dir_contents(generated_dest_path_ts);

    // copy generated rs models into destination folder
    let src_models_rs = format!("{}/src/models", generated_src_path_rs);
    copy_dir_content(src_models_rs.as_str(), generated_dest_path_rs);

    // copy generated ts models into destination folder
    let src_models_ts = format!("{}/model", generated_src_path_ts);
    copy_dir_content(src_models_ts.as_str(), generated_dest_path_ts);

    p!(">>> finished build.");
}

// run command; it exits the programm if exit code not 0
// panic! when command execution failed
fn run_cmd(cmd: &mut Command, cmd_name: &str, err_msg: &str) {
    let cmd_output = cmd
        .stdout(Stdio::piped())
        .output()
        .unwrap_or_else(|_| panic!("failed to execute {} command", cmd_name));

    let cmd_exit_code = cmd_output
        .status
        .code()
        .unwrap_or_else(|| panic!("{} command: should return exit code", cmd_name));

    if cmd_exit_code != 0 {
        p!("{}", err_msg);

        if !cmd_output.stderr.is_empty() {
            let stderr_str = std::str::from_utf8(&cmd_output.stderr).unwrap();
            p!("{}", stderr_str);
        }

        exit(1);
    }
}

// creates folder if missing
// panic! when folder couldn't be created
fn create_dir_if_missing(dir_path: &str) {
    let create_res = fs::create_dir_all(dir_path);
    if create_res.is_err() {
        p!(
            "Failed to create a dir: {}: {}",
            dir_path,
            create_res.err().unwrap()
        );
        exit(1);
    }
}

// remove content from directory but not the directory itself
// panic! when content couldn't be removed
fn remove_dir_contents(dir_path: &str) {
    // traverse folder
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            // go trough each file in folder
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    let dir_removal_res = fs::remove_dir_all(&path);
                    if dir_removal_res.is_err() {
                        p!(
                            "Failed to remove a dir {:?}: {}",
                            path,
                            dir_removal_res.err().unwrap()
                        );
                        exit(1);
                    }
                } else {
                    let file_removal_res = fs::remove_file(&path);
                    if file_removal_res.is_err() {
                        p!(
                            "Failed to remove a file {:?}: {}",
                            path,
                            file_removal_res.err().unwrap()
                        );
                        exit(1);
                    }
                }
            }
        }
        Err(e) => {
            p!("Failed to read {}: {}", dir_path, e);
            exit(1);
        }
    }
}

// copy content of source folder into destination folder
// panic! when content couldn't be copied
fn copy_dir_content(source_path: &str, destination_path: &str) {
    // traverse source folder
    match fs::read_dir(source_path) {
        Ok(entries) => {
            // go trough each file in source folder
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        // create file in destination folder
                        let file_name_os = entry.file_name();
                        let file_name = file_name_os.to_str().unwrap();
                        let dest_file_path = format!("{}/{}", destination_path, file_name);
                        let dest_file_res = File::create(dest_file_path.clone());
                        if dest_file_res.is_err() {
                            p!(
                                "Failed to create destination file {}: {}",
                                file_name,
                                dest_file_res.err().unwrap()
                            );
                            exit(1);
                        }

                        // copy content from source file into destination file
                        copy_file_content(entry.path().to_str().unwrap(), dest_file_path.as_str());
                    }
                    Err(e) => {
                        p!("Failed to read dir entry in {}: {}", source_path, e);
                        exit(1);
                    }
                }
            }
        }
        Err(e) => {
            p!("Failed to read {}: {}", source_path, e);
            exit(1);
        }
    }
}

// copy content of source file into destination file
// panic! when content couldn't be copied
fn copy_file_content(src_file: &str, dest_file: &str) {
    let copy_res = fs::copy(src_file, dest_file);
    if copy_res.is_err() {
        p!(
            "Failed to copy content from source file {} into destination file {}",
            src_file,
            dest_file
        );
        exit(1);
    }
}
