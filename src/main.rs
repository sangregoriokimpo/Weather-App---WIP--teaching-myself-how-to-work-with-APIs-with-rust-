use std::fmt::format;
use std::io;
use serde::Deserialize;
use colored::*;
use reqwest::Response;
//small app to teach myself how to work with APIs
//Struct that deserializes JSON response from  openWeatherMapAPI
#[derive(Deserialize,Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

//Struct to serve as weather description
#[derive(Deserialize,Debug)]
struct Weather{
    description: String,
}

#[derive(Deserialize,Debug)]
struct Main{
    temp: f64,
    humidity: f64,
    pressure: f64,
}

//Struct to represent wind information
#[derive(Deserialize,Debug)]
struct Wind{
    speed: f64,
}

//Function pull information from weatherapp API
fn getWeatherInfo(city: &str,country_code: &str,api_key:&str) ->
Result<WeatherResponse,reqwest::Error>{
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",city,country_code,api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

//Function to display weather information
fn displayWeatherInfo(response: &WeatherResponse){
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}ºC,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        getTemperatureEmoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    //Coloring the weather text based on weather conditions
    let weatherTextColored = match description.as_str(){
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    //print the colored weather information
    println!("{}",weatherTextColored);

    fn getTemperatureEmoji(temperature: f64) -> &'static str{
        if temperature < 0.0{
            "❄️"
        }else if temperature >= 0.0 && temperature < 10.0{
            "☁️"
        }else if temperature >= 10.0 && temperature < 20.0{
            "⛅️"
        }else if temperature >= 20.0 && temperature < 30.0 {
            "🌤️"
        }else{
            "🔥"
        }
    }
}

fn main() {
    println!("{}","Weather Application".bright_yellow());
    loop {
        //Input city
        println!("{}","Enter name of city:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input");
        let city = city.trim();

        //Input country code
        println!("{}","Enter country code:".bright_green());
        let mut country = String::new();
        io::stdin().read_line(&mut country).expect("Failed to read input");
        let country = country.trim();

        //Get API key
        let api_key = "08241f2fc333dfd76872c97caf0e4273";

        //Call get weather info function
        match getWeatherInfo(city,country,api_key) {
            Ok(response) => {
                displayWeatherInfo(&response);
            }
            Err(err) => {
                eprintln!("Error: {}",err);
            }
        }

        println!("{}","Search for another city? (y/n)".bright_green());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "y"{
            println!("Exiting Weather application");
            break;
        }
    }
}



