use raven::*;
use std::fs;
use std::path::PathBuf;

const UI: &str = include_str!("../ui.xml");

fn key_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("sem diretório home");
    PathBuf::from(home).join(".ponto-admin").join("key.txt")
}

fn load_secret() -> Option<String> {
    fs::read_to_string(key_path())
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn set_label(window: Rid, name: &str, value: &str) {
    unsafe { Rlabel_set_text(widget_find(window, name), cstr(value).as_ptr()) };
}

fn set_pubkey_label(window: Rid, public_hex: &str) {
    let msg = if public_hex.is_empty() {
        "(sem chaves — gere o par)".to_string()
    } else {
        format!("Chave pública:\n{public_hex}")
    };
    set_label(window, "pubkey", &msg);
}

fn set_status(window: Rid, msg: &str) {
    set_label(window, "status", msg);
}

fn generate_keys() -> Result<(String, String), String> {
    let (secret, public) = paid_core::keygen()?;
    let path = key_path();
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&path, &secret).map_err(|e| e.to_string())?;
    Ok((secret, public))
}

fn keygen_cli() {
    match generate_keys() {
        Ok((_, public)) => {
            println!("Chaves geradas:");
            println!("  privada: {}", key_path().display());
            println!("  pública: {public}");
            println!("Cole a chave pública em src-tauri/paid_public_key.txt e recompile o app.");
        }
        Err(e) => {
            eprintln!("Falha ao gerar chaves: {e}");
            std::process::exit(1);
        }
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("keygen") {
        keygen_cli();
        return;
    }

    unsafe { Rinit(R_BACKEND_VULKAN) };
    let window = window_from_xml(UI);

    if let Some(secret) = load_secret() {
        set_pubkey_label(window, &paid_core::public_from_secret(&secret).unwrap_or_default());
    } else {
        set_pubkey_label(window, "");
    }

    action_bind("app.keygen", move |_sender, _action| {
        match generate_keys() {
            Ok((_, public)) => {
                set_pubkey_label(window, &public);
                set_status(window, "Chaves geradas e salvas. Cole a chave pública no app e recompile.");
            }
            Err(e) => set_status(window, &format!("Falha: {e}")),
        }
    });

    action_bind("app.sign", move |_sender, _action| {
        let Some(secret) = load_secret() else {
            set_status(window, "Sem chave privada — gere o par de chaves.");
            return;
        };
        let device = unsafe { text(Rtext_input_get_text(widget_find(window, "device"))) };
        let until = unsafe { text(Rtext_input_get_text(widget_find(window, "until"))) };
        if device.trim().is_empty() {
            set_status(window, "Informe o ID do dispositivo.");
            return;
        }
        let file = match paid_core::sign(&secret, device.trim(), until.trim()) {
            Ok(f) => f,
            Err(e) => {
                set_status(window, &e);
                return;
            }
        };
        let path = rfd::FileDialog::new()
            .set_title("Salvar comprovante de pagamento")
            .set_file_name(&format!("pago-{}-{}.ponto", file.paid_up_to, file.device))
            .add_filter("Comprovante Ponto", &["ponto"])
            .save_file();
        match path {
            Some(path) => {
                let json = serde_json::to_string_pretty(&file).unwrap();
                match fs::write(&path, json) {
                    Ok(()) => set_status(window, &format!("Comprovante salvo: {}", path.display())),
                    Err(e) => set_status(window, &format!("Falha ao salvar: {e}")),
                }
            }
            None => set_status(window, "Cancelado."),
        }
    });

    unsafe {
        while Rwait_frame(-1.0) != 0 {}
        Rshutdown();
    }
}
