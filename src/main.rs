use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};
use color_eyre::Result;
use futures::StreamExt;
use glob::{GlobError, PatternError};
use std::{
    io::{stdout, Write},
    vec,
};

#[tokio::main]
async fn main() -> Result<()> {
    let msgs_context = read_messages()?;
    eprintln!("{:?}", msgs_context);

    let msg_intro = ChatCompletionRequestMessageArgs::default().role(Role::User).content("A continuación se muestra el temario de una asignatura llamada Metodología de la investigación").build()?;
    let msg_final = ChatCompletionRequestMessageArgs::default().role(Role::User).content("Necesito que me hagas un glosario de la asignatura. Este glosario contendrá una única palabra, seguida de una pequeña descripción. Necestio al menos 20 ítems del glosario").build()?;

    eprintln!("======");

    let mut msgs = vec![];
    msgs.extend_from_slice(&[msg_intro]);
    msgs.extend_from_slice(&msgs_context);
    msgs.extend_from_slice(&[msg_final]);

    let completion = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(msgs)
        .build()?;

    let client = Client::new();

    let mut stream = client.chat().create_stream(completion).await?;

    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        let response = result?;
        response.choices.iter().for_each(|chat_choice| {
            if let Some(ref content) = chat_choice.delta.content {
                write!(lock, "{}", content).unwrap();
            }
        });
        stdout().flush()?;
    }

    Ok(())
}

fn read_messages() -> Result<Vec<ChatCompletionRequestMessage>> {
    let base_path = std::env::current_dir()?;
    let glob = format!("{}/priv/input.txt", base_path.to_str().unwrap());
    eprintln!("glob: {:?}", glob);

    let paths = glob::glob(&glob)?.collect::<Result<Vec<_>, _>>()?;
    eprintln!("paths: {:?}", paths);

    let result = paths
        .iter()
        .flat_map(|path| std::fs::read_to_string(path))
        .map(|contents| {
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(contents)
                .build()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(result)
}
