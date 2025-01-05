use::std::io;

fn main() {
    println!("Insert the temperature in celcius");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    
    let temp : f32 = temp.trim().parse().expect("Please input a valid number!");

    let temp = fahrenheit_to_celsius(temp);

    println!("The temperature in Fahrenheit is {temp} degrees");

}

fn fahrenheit_to_celsius(temp:f32)-> f32{
    temp * (9.0/5.0) + 32.0
}