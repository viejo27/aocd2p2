use std::fs::read_to_string;

fn main() {
    let mut sum: Option<u32> = Some(0);

    for line in read_to_string("./src/input.txt").unwrap().lines() {
        // println!("{}",line);
        let mut cube_cant: Option<u32> = Some(0);
        let mut word_iterator = line.split_whitespace();
        let mut red: Option<u32> = None;
        let mut green: Option<u32> = None;
        let mut blue: Option<u32> = None;
        let mut red_total: Option<u32> = None;
        let mut green_total: Option<u32> = None;
        let mut blue_total: Option<u32> = None;
        if word_iterator.next() == Some("Game") {
        }
        while let Some(word) = word_iterator.next() {
            let is_number: Result<u32,_> = word.to_string().parse();
            match is_number {
                Ok(cubes) => cube_cant = Some(cubes),
                Err(_err) => {
                    if word.ends_with(",") {
                        let cube_colour = word.trim_end_matches(",");
                        match cube_colour {
                            "red" => {
                                red = Some(red.unwrap_or(0) + cube_cant.unwrap_or(0));
                            },
                            "green" => {
                                green = Some(green.unwrap_or(0) + cube_cant.unwrap_or(0));
                            }
                            "blue" => {
                                blue = Some(blue.unwrap_or(0) + cube_cant.unwrap_or(0));
                            }
                            _ => {}
                        }
                        if red_total == None && red != None {
                            red_total = Some(red.unwrap_or(0));
                        } else if red > red_total {
                            red_total = Some(red.unwrap_or(0));
                        }
                        if green_total == None && green != None {
                            green_total = Some(green.unwrap_or(0));
                        } else if green > green_total {
                            green_total = Some(green.unwrap_or(0));
                        }
                        if blue_total == None && blue != None {
                            blue_total = Some(blue.unwrap_or(0));
                        } else if blue > blue_total {
                            blue_total = Some(blue.unwrap_or(0));
                        }
                    }
                    if word.ends_with(";") {
                        let cube_colour = word.trim_end_matches(";");
                        match cube_colour {
                            "red" => {
                                red = Some(red.unwrap_or(0) + cube_cant.unwrap_or(0));
                            },
                            "green" => {
                                green = Some(green.unwrap_or(0) + cube_cant.unwrap_or(0));
                            }
                            "blue" => {
                                blue = Some(blue.unwrap_or(0) + cube_cant.unwrap_or(0));
                            }
                            _ => {}
                        };
                        if red_total == None && red != None {
                            red_total = Some(red.unwrap_or(0));
                        } else if red > red_total {
                            red_total = Some(red.unwrap_or(0));
                        }
                        if green_total == None && green != None {
                            green_total = Some(green.unwrap_or(0));
                        } else if green > green_total {
                            green_total = Some(green.unwrap_or(0));
                        }
                        if blue_total == None && blue != None {
                            blue_total = Some(blue.unwrap_or(0));
                        } else if blue > blue_total {
                            blue_total = Some(blue.unwrap_or(0));
                        }
                        red = None;
                        green = None;
                        blue = None;
                    }
                    match word {
                        "red" => {
                            red = Some(red.unwrap_or(0) + cube_cant.unwrap_or(0));
                            if red_total == None && red != None {
                                red_total = Some(red.unwrap_or(0));
                            } else if red > red_total {
                                red_total = Some(red.unwrap_or(0));
                            }
                            red = None;
                            green = None;
                            blue = None;
                        },
                        "green" => {
                            green = Some(green.unwrap_or(0) + cube_cant.unwrap_or(0));
                            if green_total == None && green != None {
                                green_total = Some(green.unwrap_or(0));
                            } else if green > green_total {
                                green_total = Some(green.unwrap_or(0));
                            }
                            red = None;
                            green = None;
                            blue = None;
                        },
                        "blue" => {
                            blue = Some(blue.unwrap_or(0) + cube_cant.unwrap_or(0));
                            if blue_total == None && blue != None {
                                blue_total = Some(blue.unwrap_or(0));
                            } else if blue > blue_total {
                                blue_total = Some(blue.unwrap_or(0));
                            }
                            red = None;
                            green = None;
                            blue = None;
                        },
                        _ => {},
                    }
                }
            }
        }
        if red_total != None || green_total != None || blue_total != None {
            // println!("red_final {:?}",red_total);
            // println!("green_final {:?}",green_total);
            // println!("blue_final {:?}",blue_total);
            let line_power = Some(red_total.unwrap_or(1) * green_total.unwrap_or(1) * blue_total.unwrap_or(1));
            // println!("line_power {:?}",line_power);
            sum = Some(sum.unwrap_or(0) + line_power.unwrap_or(0));
        }
        red_total = None;
        green_total = None;
        blue_total = None;
    }
    println!("total is {:?}",sum);
}
