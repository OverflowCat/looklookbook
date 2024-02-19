use diesel::{result::Error, Connection};
use dotenvy::dotenv;
use models::{NewChap, NewRecord, NewReminder, NewUser};
use std::time::{Duration, SystemTime};
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::{prelude::*, utils::command::BotCommands};
pub mod db;
pub mod models;
pub mod schema;

use db::{
    book::*,
    chap::*,
    get_conn,
    record::*,
    reminder::{create_reminder, delete_reminder, get_default_reminders},
    user::*,
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    init(bot.clone()).await;
    Command::repl(bot, answer).await;
}

async fn cui(bot: Bot, user: models::User, interval: Duration) {
    let uid = user.uid;
    tokio::spawn(async move {
        loop {
            let res = bot
                .send_message(
                    ChatId(-1000000000),
                    format!(
                        "[{}](tg://user?id={uid})，你该读书了！\n\n下次提醒将在 {} 分钟后",
                        user.username,
                        interval.as_secs() / 60
                    ),
                )
                .parse_mode(MarkdownV2)
                .await;
            if let Err(e) = res {
                println!("催 user {uid} 失败！\n{e}");
            }
            tokio::time::sleep(interval).await;
        }
    });
}

async fn init(bot: Bot) {
    let mut conn_guard = get_conn().await.lock().await;
    let conn = &mut *conn_guard;
    let users = get_all_users(conn).unwrap();
    println!("Got {} users", users.len());
    // let reminders = users
    //     .iter()
    //     .filter_map(|user| get_default_reminders(conn, user.uid).ok())
    //     .collect::<Vec<_>>();
    // println!("Got {} reminders", reminders.len());
    for user in users {
        if let Ok(reminder) = get_default_reminders(conn, user.uid) {
            if let Some(mut interval) = reminder.interval {
                if interval < 3600 {
                    interval = 3600;
                }
                let bot = bot.clone();
                tokio::spawn(
                    async move { cui(bot, user, Duration::from_secs(interval as u64)).await },
                );
            }
        }
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "支持的命令如下：")]
enum Command {
    #[command(description = "我是机器人，跟我聊聊天吧！")]
    /** Start the bot */
    Start,
    #[command(description = "显示此文本")]
    /** Show the help text */
    Help,
    #[command(description = "开始阅读计时")]
    /** Start a new reading record */
    Checkin(String),
    #[command(description = "结束计时并记录本次阅读")]
    /** Stop and save the current reading record */
    Checkout,
    #[command(description = "放弃当前的计时")]
    /** Discard the current reading record */
    Abandon,
    #[command(description = "立下完成这本书的目标")]
    /** Set a goal for a book */
    Flag,
    #[command(description = "查看自己的目标及完成情况")]
    /** Show the goals and progress of the user */
    MyFlags,
    #[command(description = "查看别人的目标及完成情况")]
    OthersFlags,
    #[command(description = "比较谁读得更多")]
    Board,
    #[command(description = "查看自己的阅读记录")]
    Me,
    #[command(description = "提醒我读书")]
    Cuiwo(u32),
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
                .unwrap()
                .current_rid;
                if let Some(_) = current_rid {
                    bot.send_message(
                        msg.chat.id,
                        "你有未结束的阅读记录，请先结束或放弃当前的计时",
                    )
                    .await?
                } else {
                    let (bid, cid, rid) = conn
                        .transaction::<(_, _, _), Error, _>(|conn| {
                            let bid = create_or_get_book(conn, major)?;
                            let cid = create_or_get_chap(
                                conn,
                                NewChap {
                                    bid,
                                    cid: None,
                                    creator_uid: id.0 as i64,
                                    heading: &minor.to_string(),
                                },
                            )?;

                            let rec = create_and_get_record(
                                conn,
                                NewRecord {
                                    rid: None,
                                    uid: id.0 as i64,
                                    cid,
                                    fromtime: SystemTime::now()
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs()
                                        as i64,
                                    totime: None,
                                },
                            )?;
                            let rid = rec.rid;
                            update_current_record(conn, id.0 as i64, rid)?;
                            Ok((bid, cid, rid))
                        })
                        .unwrap();

                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "你开始阅读了 {major} 书 ({bid}) 的 {minor} 章节 ([{cid}])，记录编号为 {}",
                            rid.unwrap()
                        ),
                    )
                    .await?
                }
            }
        }
        Command::Checkout => {
            let user = msg.from().unwrap();
            let uid = user.id.0 as i64;
            let mut conn_guard = get_conn().await.lock().await;
            let conn = &mut *conn_guard;
            let userdata = create_or_get_user(
                conn,
                NewUser {
                    uid,
                    username: &user.username.clone().unwrap_or_default(),
                    current_rid: None,
                },
            )
            .unwrap();
            if let Some(rid) = userdata.current_rid {
                let totime = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                let fromtime = conn
                    .transaction::<_, Error, _>(|conn| {
                        let fromtime = finish_record(conn, rid, totime)?;
                        update_current_record(conn, uid, None)?;
                        Ok(fromtime)
                    })
                    .unwrap();
                let duration = Duration::from_secs((totime - fromtime) as u64).as_secs();
                let min = duration / 60;
                let sec = duration % 60;
                bot.send_message(
                    msg.chat.id,
                    format!("你结束了本次阅读，共计 {} 分 {} 秒", min, sec),
                )
                .await?
            } else {
                bot.send_message(msg.chat.id, "结束计时").await?
            }
        }
        Command::Abandon => {
            let user = msg.from().unwrap();
            let id = user.id;
            let mut conn_guard = get_conn().await.lock().await;
            let conn = &mut *conn_guard;
            let userdata = create_or_get_user(
                conn,
                NewUser {
                    uid: id.0 as i64,
                    username: &user.username.clone().unwrap_or_default(),
                    current_rid: None,
                },
            )
            .unwrap();
            if let Some(rid) = userdata.current_rid {
                conn.transaction::<_, Error, _>(|conn| {
                    delete_record(conn, rid)?;
                    update_current_record(conn, id.0 as i64, None)?;
                    Ok(())
                })
                .unwrap();
                bot.send_message(msg.chat.id, "你放弃了当前的计时").await?
            } else {
                bot.send_message(msg.chat.id, "你没有正在进行的计时")
                    .await?
            }
        }
        Command::Cuiwo(secs) => {
            let user = msg.from().unwrap();
            let uid = user.id.0 as i64;
            let mut conn_guard = get_conn().await.lock().await;
            let conn = &mut *conn_guard;
            let user_data = create_or_get_user(
                conn,
                NewUser {
                    uid,
                    username: &user.username.clone().unwrap_or_default(),
                    current_rid: None,
                },
            )
            .unwrap();
            let _ = delete_reminder(conn, uid);
            create_reminder(
                conn,
                NewReminder {
                    id: None,
                    uid,
                    bid: None,
                    cron: None,
                    interval: Some(secs.into()),
                },
            )
            .unwrap();
            cui(bot.clone(), user_data, Duration::from_secs(secs.into())).await;
            bot.send_message(
                msg.chat.id,
                format!("好的，我会在 {} 分钟后提醒你读书", secs / 60),
            )
            .await?
        }
        Command::Biecuiwole => {
            let user = msg.from().unwrap();
            let uid = user.id.0 as i64;
            let mut conn_guard = get_conn().await.lock().await;
            let conn = &mut *conn_guard;
            let _ = delete_reminder(conn, uid);
            bot.send_message(msg.chat.id, "好的，我不会再提醒你读书了")
                .await?
        }
        Command::Me => {
            let user = msg.from().unwrap();
            let id = user.id;
            let mut conn_guard = get_conn().await.lock().await;
            let conn = &mut *conn_guard;
            let userdata = create_or_get_user(
                conn,
                NewUser {
                    uid: id.0 as i64,
                    username: &user.username.clone().unwrap_or_default(),
                    current_rid: None,
                },
            )
            .unwrap();
            let uid = userdata.uid;
            let sum = get_duration_sum(conn, uid).unwrap_or_default().as_secs();
            bot.send_message(
                msg.chat.id,
                format!("你总共阅读了 {} 分 {} 秒", sum / 60, sum % 60),
            )
            .await?
        }
        Command::Board => {
            let mut conn_guard = get_conn().await.lock().await;
            let conn = &mut *conn_guard;
            let res = get_duration_sum_rank(conn, 0).unwrap();
            let mut s = String::new();
            for (duration, user) in res {
                let name = if user.username == "" {
                    "无名氏"
                } else {
                    &user.username
                };
                s.push_str(&format!(
                    "{name} [{}]: {} 分 {} 秒\n",
                    user.uid,
                    duration.as_secs() / 60,
                    duration.as_secs() % 60
                ))
            }
            bot.send_message(msg.chat.id, s).await?
        }
        _ => bot.send_message(msg.chat.id, "Unimplemented…").await?,
    };

    Ok(())
}
