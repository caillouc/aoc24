use std::{fmt::Debug, str::FromStr};

#[allow(dead_code)]
pub fn parse_double_vec(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect()).collect()
}

#[allow(dead_code)]
pub fn parse_double_vec_int(data: &str) -> Vec<Vec<u32>> {
    data.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

#[allow(dead_code)]
pub fn parse_double_vec_delimiter<T>(data: &str, delimiter: &str) -> Vec<Vec<T>> 
where T: FromStr, <T as FromStr>::Err: Debug
{
    data.lines().map(|l| {
        l.split(delimiter).map(|e| e.parse::<T>().unwrap()).collect()
    }).collect()
}

#[allow(dead_code)]
pub fn parse_vec_delimiter<T>(data: &str, delimiter: &str) -> Vec<T> 
where T: FromStr, <T as FromStr>::Err: Debug
{
    if data.lines().count() == 1 {
        return data.lines().next().unwrap().split(delimiter).map(|e| e.parse::<T>().unwrap()).collect()
    }
    data.lines().map(|l| l.parse::<T>().unwrap()).collect()
}

#[allow(dead_code)]
pub fn parse_vec_char(data: &str) -> Vec<char> {
    data.lines().flat_map(|l| l.chars()).collect()
}