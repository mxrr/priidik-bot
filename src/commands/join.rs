use std::{future::Future, pin::Pin};
use std::sync::Arc;
use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
  model::id::GuildId,
};
use songbird::Songbird;
use rand::Rng;
use tokio::time::sleep;
use std::time::Duration;

pub struct JoinCommand {
  name: &'static str,
}

impl JoinCommand {
  pub fn new() -> Self {
    Self {
      name: "join",
    }
  }
}

#[async_trait]
impl Command for JoinCommand {
  fn name(&self) -> &'static str {
    self.name
  }

  fn requirement(&self, _ctx: &Context, msg: &Message) -> bool {
    msg.content == "&join"
  }

  async fn action(&self, ctx: Context, msg: Message) {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
      .voice_states.get(&msg.author.id)
      .and_then(|vs|vs.channel_id);

    let connect_to = match channel_id {
      Some(ch) => ch,
      None => {
        if let Err(err) = msg.reply(&ctx.http, "Mis see on").await {
          println!("Error: {:?}", err);
        }
        return;
      }
    };

    let manager = songbird::get(&ctx).await
      .expect("Songbird client error").clone();

    let _handler = manager.join(guild_id, connect_to).await;
    {
      let queue_lock = {
        let data = ctx.data.read().await;
        data.get::<crate::Queue>()
          .expect("No queue")
          .clone()
      };
    
    
      let mut queue = queue_lock.write().await;
      let current_time = crate::utils::get_current_time();

      let secs_to_wait = rand::thread_rng().gen_range(3..15);
      let data = crate::queue::VoiceLineData {
        msg: msg.clone(),
        ctx: ctx.clone(),
        prev_time: current_time,
        new_time: current_time + secs_to_wait,
        time_spent: 0,
        manager,
      };
      queue.insert(guild_id.into(), data.clone());
      crate::queue::play_voiceline(data, guild_id).await;
    }

    //play_voiceline(ctx.clone(), manager, guild_id, msg.clone(), Duration::new(0, 420)).await;

    
    

    self.log(ctx, msg);
  }
}


/* fn old_play_voiceline(
  ctx: Context, 
  manager: Arc<Songbird>, 
  guild_id: serenity::model::id::GuildId,
  msg: Message,
  prev_timer: Duration
) -> Pin<Box<dyn Future<Output = ()> + Send>> {
  Box::pin(async move {
    if let Some(handler_lock) = manager.get(guild_id) {

      let roll: i8 = rand::thread_rng().gen_range(1..10);
  
      let filename = format!(
        "mis{}.mp3", 
        if roll < 10 { 
          format!("0{}", roll)
        } else { 
          roll.to_string() 
        }
      );
      let path_str = format!("./audio/{}", filename);
      let path = std::path::Path::new(&path_str);
      let source = match songbird::ffmpeg(path).await {
        Ok(source) => source,
        Err(err) => {
          println!("Error: {:?}", err);
          if let Err(err) = msg.channel_id.say(&ctx.http, "ffmpeg error").await {
            println!("Error: {:?}", err);
          }
          return;
        },
      };
  
      let mut handler = handler_lock.lock().await;
      let _handle = handler.play_source(source);

      let total = prev_timer.as_secs();
      let mins = total / 60;
      let secs = total - mins * 60;

      let content = format!(
        "mis see on
        ||{}m {}s||", 
        mins, 
        secs 
      );

      if let Err(err) = msg.channel_id.say(&ctx.http, content).await {
        println!("Error: {:?}", err);
      }
    } else {
      return;
    }

    sleep(Duration::new(1, 500_000_000)).await;
    match ctx.cache.channel(VANAISA_ID).await {
      Some(channel) => {
        if let Err(err) = channel.id().say(&ctx.http, "(mis see on)").await {
          println!("Error posting in comms channel: {:?}", err);
        }
      },
      None => println!("Couldn't find comms channel"),
    }



    let secs_to_wait = rand::thread_rng().gen_range(3..1500);
    let sleep_timer = Duration::new(secs_to_wait, 420);
    
    sleep(sleep_timer).await;
    play_voiceline(ctx, manager, guild_id, msg, sleep_timer).await;
  })
} */

