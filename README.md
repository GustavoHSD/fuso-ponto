# Fuso Ponto

App de ponto (time tracker) feito com Tauri + React + TypeScript.

## Marcar horas como pagas (fluxo offline com arquivo assinado)

Cada app é uma identidade (ID do dispositivo, em **Ajustes**). O administrador
emite um **comprovante de pagamento assinado** (ed25519) e o funcionário
importa o arquivo no app. A chave pública está embutida no app
(`src-tauri/paid_public_key.txt`); a chave privada fica só com o administrador.

### Administrador

1. Clone a biblioteca Raven e construa (uma vez):
   ```
   git clone https://github.com/oscar-soirey/Raven.git admin/Raven
   cd admin/Raven && cmake -B build -DRAVEN_BUILD_SHARED=ON -DRAVEN_BUILD_EXAMPLES=OFF -DRAVEN_BUILD_TESTS=OFF -DRAVEN_BUILD_TOOLS=OFF && cmake --build build --parallel
   ```
2. Gere o par de chaves (uma vez): `cargo run -p ponto-admin keygen`
   — a privada fica em `~/.ponto-admin/key.txt`, a pública vai colada em `src-tauri/paid_public_key.txt` (recompile o app depois).
3. Abra o GUI: `cargo run -p ponto-admin` — informe o ID do dispositivo,
   a data "pago até", clique **Assinar e salvar…** e envie o arquivo `.ponto`
   ao funcionário (WhatsApp, e-mail, etc).

### Funcionário

- **Ajustes → Importar comprovante…** e escolher o arquivo `.ponto`.
- O app só aceita arquivos com assinatura válida **e** destinados ao próprio
  device ID. Arquivos antigos nunca regridem o status (vale o mais recente).
- O relatório mostra o selo verde **Pago** nos dias pagos.

## Estrutura

- `src/` — frontend React (Tauri)
- `src-tauri/` — backend Rust do app (SQLite local, comandos Tauri)
- `paid-core/` — formato do comprovante assinado (compartilhado: assina no admin, verifica no app)
- `admin/` — GUI do administrador (Rust + [Raven](https://github.com/oscar-soirey/Raven))
