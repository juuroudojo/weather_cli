use std::io;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather{
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    pressure: i32,
    humidity: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
}

fn get_weather(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}", city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
} 

fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temp = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    let weather_info: String = format!(
        "The weather in {} : {} {}
        > Temperature: {:.1}°C
        > Humidity: {:.1}%
        > Pressure: {:.1} hPa
        > Wind speed: {:.1} m/s",
        response.name, description, get_emoji(temp), temp, humidity, pressure, wind_speed
    );

    let info_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_info.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_info.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "fog" | "sand" | "dust" => weather_info.dimmed(),
        "shower rain" | "rain" | "thunderstorm" => weather_info.bright_cyan(),
        "snow" => weather_info.bright_white(),
        _ => weather_info.normal(),
    };

    println!("{}", info_colored);
}

fn get_emoji(temp:f32) -> &'static str {
    if temp < 0.0 {
        return "❄️";
    } else if temp < 10.0 {
        return "⛄️";
    } else if temp < 20.0 {
        return "🌦";
    } else if temp < 30.0 {
        return "☀️";
    } else {
        return "🔥";
    }
}

fn main() {
    println!("{}", "Welcome to Weather App!".green().bold());
    loop {
        println!("{}", "Enter the city name: ".
        bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read line");
        let city: &str = city.trim();

        println!("{}", "Enter the country code (US for United States, uk for United Kingdom, etc.): ".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read line");
        let country_code: &str = country_code.trim();

        let api_key = "3e877f7ef0700dafdb92d8a2176083fb";

        match get_weather(city, country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            },
            Err(err) => {
                println!("{}", format!("Error: {}", err).red());
            }
        }

        println!("{}", "Do you want to check the weather for another city? (yes/no): ".bright_green());

        let mut answer = String::new();

        io::stdin().read_line(&mut answer).expect("Failed to read line");

        let answer: &str = answer.trim();

        if answer != "yes" {
            break;
        }
    }
  }