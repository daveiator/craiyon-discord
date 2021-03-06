use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::CommandError;
use serenity::model::prelude::*;
use serenity::prelude::*;

use image;

use std::fs::File;
use std::io::Write;
use std::io::Cursor;

use crate::craiyon;
use crate::image_formatter;

#[command]
async fn ai(ctx: &Context, msg: &Message) -> CommandResult {
    //remove the content before the first whitespace
    let content = msg.content.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");
    let tmp_msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.content("Generating content...");
        m.reference_message(msg);
        m.allowed_mentions(|am| {
            am.replied_user(true);
            am
        });
        m
    }).await?;
    let typing = msg.channel_id.start_typing(&ctx.http)?;
    info!("Generating {:?} for {}", content, msg.author.name);
    return match craiyon::generate(content.to_string()).await {
        Ok(images) => {
            let image = image_formatter::image_collage(
                images.iter().map(|image| {
                    image::load_from_memory_with_format(image, image::ImageFormat::Jpeg).unwrap()
                }),
                            image_formatter::CollageOptions {
                                image_count: (3, 3),
                                image_size: (256, 256),
                                gap: 8,
                            },
                        );                       
            let mut buffer = Cursor::new(Vec::new());
            image.write_to(&mut buffer, image::ImageOutputFormat::Jpeg(24))?;
            let image_bytes = buffer.get_ref().to_vec();
            //save image to file
            let mut file = File::create(format!("./temp/{}.jpeg", msg.id))?;
            file.write_all(&image_bytes)?;
            info!("Sending image...");
            msg.channel_id.send_files(&ctx.http, vec![format!("temp/{}.jpeg", msg.id).as_str()], |m| {
                m.content("Images for prompt:");
                m.reference_message(msg);
                m.allowed_mentions(|am| {
                    am.replied_user(true);
                    am
                });
                m
        }).await?;
            _ = typing.stop();
            tmp_msg.delete(&ctx.http).await?;
            //delete image file
            std::fs::remove_file(format!("./temp/{}.jpeg", msg.id))?;
            Ok(())
        },
        Err(e) => {
            error!("Couldn't generate content: {}", e);
            msg.channel_id.say(&ctx.http, format!("Couldn't generate content due to error: {}", &e)).await?;
            Err(CommandError::from(e))
        }

    }
}
