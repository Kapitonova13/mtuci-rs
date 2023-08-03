use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherData {
    name: String,
    main: Main,
    weather: Vec<Weather1>,
    wind: Wind
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
}

#[derive(Debug, Deserialize)]
struct Weather1 {
    description: String,
}

#[derive(Debug, Deserialize)]
struct WeatherData1 {
    list: Vec<List>,
    city: City,   
}

#[derive(Debug, Deserialize)]
struct List {
    main: Main1,
    dt_txt: String,
    weather: Vec<Weather>,
    wind: Wind1
}

#[derive(Debug, Deserialize)]
struct Wind1 {
    speed: f64,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize)]
struct City {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Main1 {
    temp: f64,
    feels_like: f64,
}

use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Day,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Этот бот поможет узнать прогноз погоды в Москве:\n-------------------")]
enum Command {
    #[command(description = "Показать этот текст.")]
    Help,
    #[command(description = "Информация о боте.")]
    Start,
    #[command(description = "Текущий прогноз погоды.")]
    WeatherNow,
    #[command(description = "Прогноз погоды на день.")]
    WeatherDay,
    #[command(description = "Прогноз погоды на день (выбрать время, в которое придёт прогноз).")]
    WeatherDayTime,
    #[command(description = "Прогноз погоды на 5 дней.")]
    WeatherWeek,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting purchase bot...");

    let bot = Bot::new("5800044949:AAFDNIwiky9IvEdpTUE7WNv44wPpbKa3Xy4");

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Start].endpoint(start))
                .branch(case![Command::WeatherNow].endpoint(get_weather_now))
                .branch(case![Command::WeatherDay].endpoint(get_weather_day))
                .branch(case![Command::WeatherDayTime].endpoint(time))
                .branch(case![Command::WeatherWeek].endpoint(week)),
        );

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Day].endpoint(get_weather_day_time))
        .branch(dptree::endpoint(invalid_state));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
}

async fn get_weather_now(bot: Bot, msg: Message) -> HandlerResult {
    let city = "Moscow";
    let token = "1905f5e079cf0e1a02a4fd8559212e49";
    let metric = "metric";
    let lang = "ru";
   
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}&lang={}", city, token, metric, lang);

    let response: WeatherData = reqwest::get(&url).await?.json().await?;
    bot.send_message(msg.chat.id, format!("Текущий прогноз погоды в городе {}\n-------------------\nТемпература: {}°C (ощущается как {}°C)\nМаксимальная температура: {}°C\nМинимальная температура: {}°C\nПогодные условия: {}\nСкорость ветра: {} м/с", response.name, response.main.temp, response.main.feels_like, response.main.temp_max, response.main.temp_min, response.weather[0].description, response.wind.speed)).await?;

    Ok(())
}

async fn week(bot: Bot, msg: Message) -> HandlerResult {
    let city = "Moscow";
    let token = "1905f5e079cf0e1a02a4fd8559212e49";
    let metric = "metric";
    let lang = "ru";

    let url = format!("http://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units={}&lang={}", city, token, metric, lang);

    let week: WeatherData1 = reqwest::get(&url).await?.json().await?;

    bot.send_message(msg.chat.id, format!("Прогноз погоды на 5 дней в городе {}", week.city.name)).await?;

    for i in week.list {
        bot.send_message(msg.chat.id, format!("-------------------\nДата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с", i.dt_txt, i.main.temp, i.main.feels_like, i.weather[0].description, i.wind.speed)).await?;
    }
    Ok(())
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Хотите узнать прогноз погоды в Москве?\nВведите /help, чтобы узнать, какие команды существуют.").await?;
    Ok(())
}

async fn get_weather_day(bot: Bot, msg: Message) -> HandlerResult {
    let city = "Moscow";
    let token = "1905f5e079cf0e1a02a4fd8559212e49";
    let metric = "metric";
    let lang = "ru";

    let url = format!("http://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units={}&lang={}", city, token, metric, lang);

    let weather1: WeatherData1 = reqwest::get(&url).await?.json().await?;
    let a1 = weather1.list[0].main.temp;
    let a2 = weather1.list[1].main.temp;
    let a3 = weather1.list[2].main.temp;
    let a4 = weather1.list[3].main.temp;
    let a5 = weather1.list[4].main.temp;
    let a6 = weather1.list[5].main.temp;

    let t1 = &weather1.list[0].dt_txt;
    let t2 = &weather1.list[1].dt_txt;
    let t3 = &weather1.list[2].dt_txt;
    let t4 = &weather1.list[3].dt_txt;
    let t5 = &weather1.list[4].dt_txt;
    let t6 = &weather1.list[5].dt_txt;

    let w1 = &weather1.list[0].weather[0].description;
    let w2 = &weather1.list[1].weather[0].description;
    let w3 = &weather1.list[2].weather[0].description;
    let w4 = &weather1.list[3].weather[0].description;
    let w5 = &weather1.list[4].weather[0].description;
    let w6 = &weather1.list[5].weather[0].description;

    let f1 = weather1.list[0].main.feels_like;
    let f2 = weather1.list[1].main.feels_like;
    let f3 = weather1.list[2].main.feels_like;
    let f4 = weather1.list[3].main.feels_like;
    let f5 = weather1.list[4].main.feels_like;
    let f6 = weather1.list[5].main.feels_like;

    let s1 = weather1.list[0].wind.speed;
    let s2 = weather1.list[1].wind.speed;
    let s3 = weather1.list[2].wind.speed;
    let s4 = weather1.list[3].wind.speed;
    let s5 = weather1.list[4].wind.speed;
    let s6 = weather1.list[5].wind.speed;

    let k = weather1.city.name;

    bot.send_message(msg.chat.id, format!("Прогноз погоды в городе {}\n-------------------\nДата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\nДата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\n\
    Дата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\nДата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\n\
    Дата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\nДата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с", 
    k, t1, a1, f1, w1, s1, t2, a2, f2, w2, s2, t3, a3, f3, w3, s3, t4, a4, f4, w4, s4, t5, a5, f5, w5, s5, t6, a6, f6, w6, s6)).await?;
    
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Введите /help, чтобы увидеть существующие команды.")
        .await?;
    Ok(())
}

async fn time(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Выберите время, в которое придёт прогноз погоды. (0-23)").await?;
    dialogue.update(State::Day).await?;
    Ok(())   
}

use chrono::{Timelike, Utc};
use std::thread;
use std::time::Duration;
async fn get_weather_day_time(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let city = "Moscow";
    let token = "1905f5e079cf0e1a02a4fd8559212e49";
    let metric = "metric";
    let lang = "ru";

    let url = format!("http://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units={}&lang={}", city, token, metric, lang);

    let weather1: WeatherData1 = reqwest::get(&url).await?.json().await?;
    let a1 = weather1.list[0].main.temp;
    let a2 = weather1.list[1].main.temp;
    let a3 = weather1.list[2].main.temp;
    let a4 = weather1.list[3].main.temp;
    let a5 = weather1.list[4].main.temp;
    let a6 = weather1.list[5].main.temp;

    let t1 = &weather1.list[0].dt_txt;
    let t2 = &weather1.list[1].dt_txt;
    let t3 = &weather1.list[2].dt_txt;
    let t4 = &weather1.list[3].dt_txt;
    let t5 = &weather1.list[4].dt_txt;
    let t6 = &weather1.list[5].dt_txt;

    let w1 = &weather1.list[0].weather[0].description;
    let w2 = &weather1.list[1].weather[0].description;
    let w3 = &weather1.list[2].weather[0].description;
    let w4 = &weather1.list[3].weather[0].description;
    let w5 = &weather1.list[4].weather[0].description;
    let w6 = &weather1.list[5].weather[0].description;

    let f1 = weather1.list[0].main.feels_like;
    let f2 = weather1.list[1].main.feels_like;
    let f3 = weather1.list[2].main.feels_like;
    let f4 = weather1.list[3].main.feels_like;
    let f5 = weather1.list[4].main.feels_like;
    let f6 = weather1.list[5].main.feels_like;

    let s1 = weather1.list[0].wind.speed;
    let s2 = weather1.list[1].wind.speed;
    let s3 = weather1.list[2].wind.speed;
    let s4 = weather1.list[3].wind.speed;
    let s5 = weather1.list[4].wind.speed;
    let s6 = weather1.list[5].wind.speed;

    let k = weather1.city.name;

    match msg.text().map(|text| text.parse::<u32>()) {
        Some(Ok(time)) => {
            bot.send_message(msg.chat.id,format!("Прогноз погоды придёт в {}:00.", time)).await?;
            loop {
                let now = Utc::now();
                let moscow_time = now.with_timezone(&chrono_tz::Europe::Moscow);
                    
                if moscow_time.hour() == time && moscow_time.minute() == 0 && moscow_time.second() == 0 {
                                
                    bot.send_message(msg.chat.id, format!("Прогноз погоды в городе {}\n-------------------\nДата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\nДата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\n\
                    Дата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\nДата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\n\
                    Дата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с\n-------------------\nДата {}\nТемпература: {}°C (ощущается как {}°C)\nПогодные условия: {}\nСкорость ветра: {} м/с", 
                    k, t1, a1, f1, w1, s1, t2, a2, f2, w2, s2, t3, a3, f3, w3, s3, t4, a4, f4, w4, s4, t5, a5, f5, w5, s5, t6, a6, f6, w6, s6)).await?;                        
                    break;
                }
                thread::sleep(Duration::from_secs(1));
                
                        } 
                dialogue.exit().await?;
            }
            
        _ => {
            bot.send_message(msg.chat.id, "Напишите число, которое соответсвует часу, в который придёт прогноз погоды. (0-23)").await?;
        }
    }
    Ok(())
}


