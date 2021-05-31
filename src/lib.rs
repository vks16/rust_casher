use std::thread;
use std::time::Duration;

struct Cacher<T,I,O> 
where 
    T: Fn(I) -> O,
    I: std::cmp::PartialEq + std::clone::Clone + std::marker::Copy,
    O: std::cmp::PartialEq + std::clone::Clone + std::marker::Copy
{
    calculation: T,
    value: Option<O>,
    input: Option<I>
}

impl<T,I,O> Cacher<T,I,O>
where
    T: Fn(I) -> O,
    I: std::cmp::PartialEq + std::clone::Clone + std::marker::Copy,
    O: std::cmp::PartialEq + std::clone::Clone + std::marker::Copy
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
            input: None
        }
    }

    fn value(&mut self, arg: I) -> O{
        
        match self.input {
            Some(val) if val == arg => {
                match self.value {
                    Some(v) => v,
                    None => {
                        self.input = Some(arg);
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            },
            Some(_) |
            None => {
                self.input = Some(arg);
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        simulated_expensive_calculation(num)
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        // let v1 = c.value(1);
        let v2 = c.value(2.2);
        // assert_eq!(v1, 1);
        assert_eq!(v2, 2.2);
    }
}