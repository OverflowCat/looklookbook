use dotenvy::dotenv;
use models::{NewChap, NewRecord, NewUser};
use std::time::{Duration, SystemTime};
use teloxide::{prelude::*, utils::command::BotCommands};
pub mod db;
pub mod models;
pub mod schema;

use db::{book::*, chap::*, get_conn, record::*, user::*};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "支持的命令如下：")]
enum Command {
    #[command(description = "我是机器人，跟我聊聊天吧！")]
    Start,
    #[command(description = "显示此文本")]
    Help,
    #[command(description = "开始阅读计时")]
    Checkin(String),
    #[command(description = "结束计时并记录本次阅读")]
    Checkout,
    #[command(description = "放弃当前的计时")]
    Abandon,
    #[command(description = "立下完成这本书的目标")]
    Flag,
    #[command(description = "查看自己的目标及完成情况")]
    Myflags,
    #[command(description = "查看别人的目标及完成情况")]
    Theirflags,
    #[command(description = "比较谁读得更多")]
    Board,
    #[command(description = "查看自己的阅读记录")]
    Me,
    #[command(description = "提醒我读书")]
    Cuiwo,
    #[command(description = "别催我了")]
    Biecuiwole,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                format!["你好，我是阅读计时机器人。你可以使用 /help 查看支持的命令。和我聊天吧！",],
            )
            .await?
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Checkin(s) => {
            let s: Vec<_> = s.split('\n').collect();
            if s.len() != 2 {
                bot.send_message(
                    msg.chat.id,
                    "请使用正确的格式：/checkin 书名\\n章节名\n\n书名和章节名之间用一个换行符分隔",
                )
                .await?
            } else {
                let major = s[0].trim();
                let minor = s[1].trim();
                let user = msg.from().unwrap();
                let id = user.id;

                let mut conn_guard = get_conn().await.lock().await;
                let conn = &mut *conn_guard;
                let current_rid = create_or_get_user(
                    conn,
                    NewUser {
                        uid: id.0 as i64,
                        username: &user.username.clone().unwrap_or_default(),
                        current_rid: None,
                    },
                )
                .await
                .current_rid;
                if let Some(_) = current_rid {
                    bot.send_message(
                        msg.chat.id,
                        "你有未结束的阅读记录，请先结束或放弃当前的计时",
                    )
                    .await?
                } else {
                    let bid = create_or_get_book(conn, major).await;
                    let cid = create_or_get_chap(
                        conn,
                        NewChap {
                            bid,
                            cid: None,
                            creator_uid: id.0 as i64,
                            heading: &minor.to_string(),
                        },
                    )
                    .await;

                    let rec = create_and_get_record(
                        conn,
                        NewRecord {
                            rid: None,
                            uid: id.0 as i64,
                            cid,
                            fromtime: SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs() as i64,
                            totime: None,
                        },
                    )
                    .await;
                    

                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "你开始阅读了 {}({}) 的 {} 章节，记录编号为 {}",
                            major,
                            bid,
                            minor,
                            rec.rid.unwrap()
                        ),
                    )
                    .await?
                }
            }
        }
        Command::Checkout => bot.send_message(msg.chat.id, "结束计时").await?,
        Command::Abandon => bot.send_message(msg.chat.id, "放弃计时").await?,
        Command::Flag => bot.send_message(msg.chat.id, "立下目标").await?,
        Command::Myflags => bot.send_message(msg.chat.id, "查看自己的目标").await?,
        Command::Theirflags => bot.send_message(msg.chat.id, "查看别人的目标").await?,
        _ => bot.send_message(msg.chat.id, "未知命令").await?,
    };

    Ok(())
}
