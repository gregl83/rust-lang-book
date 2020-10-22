use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, V>
    where
        T: Fn(V) -> V,
        V: Eq + Hash + Clone,
{
    calculation: T,
    value: HashMap<V, V>,
}

impl<T, V> Cacher<T, V>
    where
        T: Fn(V) -> V,
        V: Eq + Hash + Clone,
{
    fn new(calculation: T) -> Cacher<T, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: V) -> &mut V {
        let value = self.value.entry(arg.clone()).or_insert(
            (self.calculation)(arg)
        );

        value
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        assert_eq!(*v1, 1);

        let v2 = c.value(2);
        assert_eq!(*v2, 2);
    }
}