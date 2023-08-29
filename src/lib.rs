use wasm_bindgen::prelude::*;


fn fibonacci_series_until(value_to: u64) -> Vec<u64> {
    
    let mut series: Vec<u64> = Vec::new();
    series.push(1);
    series.push(2);
    let mut newfibo: u64;

    loop {
        newfibo = series[series.len() - 1] + series[series.len() - 2];        
        if newfibo > value_to {
            break;
        }
        series.push(newfibo)
    }

    series
}


#[wasm_bindgen]
pub fn fibonacci_translation(value: u64) -> String {

    if value == 0 {
        return "0".to_string();
    }

    let mut fibos       = fibonacci_series_until(value);
    let mut fibocount   = fibos.len();

    // Init array that represents bits
    let mut fiborep: Vec<char> = vec!['0'; fibocount]; 
    fiborep[0] = '1'; // The first bit can be set

    if fibos[fibocount - 1] == value {
        return fiborep.into_iter().collect();
    }

    let mut fibosum = fibos.pop().unwrap();
    fibocount -= 1;

    let mut i = 1;
    let mut newsum;
    while i <= fibocount {
        newsum = fibos[fibocount - i ] + fibosum;
        if newsum <= value {
            fibosum = newsum;
            fiborep[i] = '1';
            i += 1; // We can skip an iteration 'cause consecutive ones are not allowed
            if fibosum == value {
                break;
            }
        }
        i += 1;
    }
    fiborep.into_iter().collect()
}