use std::env;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Endereco {
    pub cep: String,
    pub logradouro: String,
    pub complemento: String,
    pub bairro: String,
    pub localidade: String,
    pub uf: String,
    pub ibge: String,
    pub gia: String,
    pub ddd: String,
    pub siafi: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cep = &args[1];
    let url = format!("https://viacep.com.br/ws/{}/json/", cep);
    let client_new = reqwest::blocking::Client::new();
    let req = client_new.get(&url).header("Accept", "application/json").send();

    match req {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Endereco>() {
                    Ok(json) => println!("Rua: {}", json.logradouro),
                    Err(_) => eprintln!("CEP {} não encontrado.", &cep),
                }
            } else {
                eprintln!("{:?}", resp.status().to_string());
            }
        },
        Err(e) => {
            eprintln!("{:?}", e);
        },
    }
}
