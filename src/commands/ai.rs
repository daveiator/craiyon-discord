use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::CommandError;
use serenity::model::prelude::*;
use serenity::prelude::*;

use serenity::model::channel::AttachmentType;

use std::io::Cursor;
use std::path::Path;
use image;

use std::fs::File;
use std::io::Write;

use reqwest::Url;

use crate::custom;
use crate::craiyon;
use crate::image_formatter;

#[command]
async fn ai(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Generating content...").await?;
    println!("Generating {:?} for {}", msg.content, msg.author.name);
    return match craiyon::generate(msg.content.to_string()).await {
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
            println!("Generated image:");
                        
            let mut buffer = Cursor::new(Vec::new());
            image.write_to(&mut buffer, image::ImageOutputFormat::Jpeg(8)).unwrap();
            let image_bytes = buffer.get_ref().to_vec();
            //save image to file
            let mut file = File::create("./temp/image.jpeg").unwrap();
            file.write_all(&image_bytes).unwrap();
            println!("Sending image...");
            msg.channel_id.send_files(&ctx.http, vec!["temp/image.jpeg"], |m| m.content("Image")).await?;
            Ok(())
        },

        Err(e) => {
            msg.channel_id.say(&ctx.http, format!("Couldn't generate content due to error: {}", &e)).await?;
            eprintln!("Couldn't generate content: {}", e);
            Err(CommandError::from(e))
        }

    }
}
