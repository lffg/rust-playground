// Muda esse nome, `Runner` é extremamente genérico e portanto ruim.
#[derive(Debug)]
pub struct RunnerOptions {
    // Note que como esses nomes são públicos, devem ser estáveis e portanto bem escolhidos.
    pub session_name: String,
    pub mem: usize,
}

pub fn run(options: RunnerOptions) {
    println!("from lib:\n{options:#?}");
}
