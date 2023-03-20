pub(crate) use std::env;

use teloxide::{prelude::*, utils::command::BotCommand};
use tokio_stream::wrappers::UnboundedReceiverStream;
use openai::{Client, Language};

// Defina as suas credenciais do OpenAI aqui.
const OPENAI_API_KEY: &str = "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";

// Defina o token do bot do Telegram aqui.
const TELEGRAM_BOT_TOKEN: &str = "6078471083:AAETgD69U_I_GPNCMfhqxPafWfuxwo5cbRg";

#[tokio::main]
async fn main() {
    // Crie uma instância do cliente OpenAI.
    let openai_client = Client::new(&env::var("OPENAI_API_KEY").unwrap_or(OPENAI_API_KEY));

    // Crie uma instância do bot do Telegram.
    let bot = Bot::new(TELEGRAM_BOT_TOKEN);

    // Crie um stream de atualizações do Telegram.
    let mut stream = UnboundedReceiverStream::new(bot.clone().polling_default().await.unwrap().0);

    // Processar cada atualização recebida do Telegram.
    while let Some(update) = stream.next().await {
        // Obtenha a mensagem da atualização, se houver.
        let message = match update {
            UpdateKind::Message(message) => message,
            _ => continue,
        };

        // Verifique se a mensagem é um comando suportado pelo bot.
        if let Some(command) = Command::parse(message.text.as_ref().map(String::as_str), "mybot") {
            match command {
                Command::Start => {
                    // Responder com uma mensagem de boas-vindas.
                    bot.send_message(message.chat_id(), "Olá! Eu sou um chatbot alimentado por GPT-3.5.").await?;
                }
                Command::Help => {
                    // Responder com uma mensagem de ajuda.
                    bot.send_message(message.chat_id(), Command::descriptions()).await?;
                }
                Command::Gpt3(input) => {
                    // Gerar uma resposta usando o GPT-3.5.
                    let response = openai_client.generate(
                        Language::English,
                        input,
                        None,
                        Some(10),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    ).await?;

                    // Responder com a resposta gerada pelo GPT-3.5.
                    bot.send_message(message.chat_id(), response.choices[0].text.as_ref().unwrap()).await?;
                }
            }
        }
    }

    Ok(())
}

// Definir os comandos suportados pelo bot.
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Estes são os comandos suportados.")]
enum Command {
    #[command(description = "Mostrar esta mensagem de ajuda.")]
    Help,
    #[command(description = "Iniciar o bot.")]
    Start,
    #[command(description = "Obter uma resposta do GPT-3.5.", parse_with = "split")]
    Gpt3(String),
}