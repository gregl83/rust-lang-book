pub mod vectors {
    use std::collections::HashMap;

    pub fn mean(values: Vec<f32>) -> f32 {
        let size = values.len() as f32;
        let mut sum = 0.0;
        for value in values {
            sum += value;
        }
        sum / size
    }

    pub fn median(values: Vec<f32>) -> f32 {
        let mut values = values.clone();
        let size = values.len() as f32;
        let midway: f32 = size / 2.0;
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if midway.fract() == 0.0 {
            (values[(midway - 1.0) as usize] + values[midway as usize]) / 2.0
        } else {
            values[midway.floor() as usize]
        }
    }

    pub fn mode(values: Vec<i32>) -> Vec<i32> {
        let mut results = vec![];
        let mut counts = HashMap::new();
        for value in values {
            let count: &mut i32 = counts.entry(value).or_insert(0);
            *count += 1;
        }
        let max: i32 = counts.iter().map(|v| *v.1).fold(0, i32::max);
        for (value, count) in counts {
            if max == count {
                results.push(value);
            }
        }
        results
    }
}
