// `lib_and_main_ex1` é o nome definido no `Cargo.toml`.
use lib_and_main_ex1::{run, RunnerOptions};

fn main() {
    // Obtenha os dados via Clap.
    // Transforme o resultado em `RunnerOptions` (aqui vou gerar manualmente).
    let options = RunnerOptions {
        session_name: "HWM :P".into(),
        mem: 1024,
    };
    run(options);
}
