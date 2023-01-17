use std::{collections::HashMap};

#[derive(Debug)]
struct MedianStruct
{
    value_if_len_divis_by_two:f32,
    value_if_len_not_divis_by_two:i32,
    len_is_divis_by_two:bool,
}

#[derive(Debug)]
struct ModeStruct
{
    vector_of_modes:Vec<i32>,
    occurrence:u32,
}

fn main()
{
    let mut vector_integers:Vec<i32> = vec![10, 78, -78, -1, 2, 1, 78, 3, 1, 11, -100];
    for value in &vector_integers
    {
        println!("{value}");
    }

    println!("============================ SEPARATOR ============================");

    let median:MedianStruct = returns_median_vector_ints(&mut vector_integers);
    for value in &vector_integers
    {
        println!("{value}");
    }
    println!("Median is {:#?}", median);

    println!("============================ SEPARATOR ============================");

    let vector_integers_2:Vec<i32> = vec![120, -1, 0, -1, -10, 0, 5, 0, -1, 120, -10, 0, 0, -1, 2, 120, -1];
    let hash_map_vec_ints:HashMap<i32, u32> = creates_hashmap_value_appearances(&vector_integers_2);
    for (key, value) in &hash_map_vec_ints
    {
        println!("{key} occurs {value} times");
    }

    let mode_vec_integers:ModeStruct = returns_mode(&hash_map_vec_ints);
    println!("{:?}", mode_vec_integers);
}

fn returns_median_vector_ints(vector_ints:&mut Vec<i32>) -> MedianStruct
{
    let mut median_struct_return:MedianStruct = MedianStruct
    {
        // One of these values will inevitably be zero, because in the case of a vector with numbers
        // of type i32, the median can be an integer (that is, the middle value) if the vector has
        // an odd number of elements, or a float (that is, the average of the middle two numbers).
        // Check the boolean value stored in the len_is_divis_by_two for help checking out which of
        // the other two values should be the median for your vector of i32 integers: if len_is_divis_by_two
        // is true when the function returns median_struct_return, consider the value_if_len_divis_by_two
        // and ignore value_if_len_not_divis_by_two. If len_is_divis_by_two is false when the function
        // returns, then consider the value_if_len_not_divis_by_two and ignore value_if_len_divis_by_two
        value_if_len_divis_by_two: 0.0,
        value_if_len_not_divis_by_two: 0,
        len_is_divis_by_two: true,
    };

    let mut i:usize = 0;
    while i < vector_ints.len()
    {
        let mut j:usize = 0;
        while j < vector_ints.len()
        {
            if vector_ints[i] > vector_ints[j] && i < j
            {
                let temp:i32 = vector_ints[i];
                vector_ints[i] = vector_ints[j];
                vector_ints[j] = temp;
            } else if vector_ints[i] < vector_ints[j] && i > j
            {
                let temp:i32 = vector_ints[i];
                vector_ints[i] = vector_ints[j];
                vector_ints[j] = temp;
            }
            j+=1;
        }
        i+=1;
    }

    if vector_ints.len() % 2 == 0
    {
        //RETURN A FLOAT
        let median_if_divis_by_two:f32;
        let first_middle_position:usize = vector_ints.len()/2;
        let second_middle_position:usize = first_middle_position + 1;

        let first_middle_value:i32= vector_ints[first_middle_position - 1];
        let second_middle_value:i32 = vector_ints[second_middle_position - 1];
        median_if_divis_by_two = ((first_middle_value as f32)+(second_middle_value as f32))/2.0;
        median_struct_return.value_if_len_divis_by_two = median_if_divis_by_two;
        median_struct_return.len_is_divis_by_two = true;// In this case, ignore the value stored in
        //median_struct_return.alue_if_len_not_divis_by_two
    } else
    {
        //RETURN AN INTEGER
        let middle_position_if_not_divis_by_two:f32;
        let median_if_not_divis_by_two:i32;
        let rate_change:f32 = 0.5;
        let temporary_middle_position:f32 = (vector_ints.len() as f32)/2.0;

        middle_position_if_not_divis_by_two = rate_change + temporary_middle_position;
        median_if_not_divis_by_two = vector_ints[(middle_position_if_not_divis_by_two as usize) - 1];
        median_struct_return.value_if_len_not_divis_by_two = median_if_not_divis_by_two;
        median_struct_return.len_is_divis_by_two = false; // In this case, ignore the value stored in
        //median_struct_return.value_if_len_divis_by_two
    }
    
    return median_struct_return;
}

fn creates_hashmap_value_appearances(vector_ints:&Vec<i32>) -> HashMap<i32, u32>
{
    let mut hash_map_value_numb_appearances:HashMap<i32, u32> = HashMap::new();

    let mut i:usize = 0;
    while i < vector_ints.len()
    {
        let mut j:usize = 0;
        let mut number_appearances:u32 = 0;
        while j < vector_ints.len()
        {
            if vector_ints[i] == vector_ints[j]
            {
                number_appearances+=1;
                hash_map_value_numb_appearances.insert(vector_ints[i], number_appearances);
            }
            j+=1;
        }
        i+=1;
    }

    return hash_map_value_numb_appearances;
}

fn returns_mode(hashmap_param:&HashMap<i32, u32>) -> ModeStruct
{
    let mut mode_to_return:ModeStruct = ModeStruct
    {
        occurrence:0,
        vector_of_modes:Vec::new(),
    };
    let mut max_amount_times_appears:u32 = 0;

    for (key, value) in hashmap_param
    {
        for (key_inner, value_inner) in hashmap_param
        {
            if value > value_inner && value > &max_amount_times_appears
            {
                max_amount_times_appears = *value;
            } else if value_inner > value && value_inner > &max_amount_times_appears
            {
                max_amount_times_appears = *value_inner;
            }
        }
        mode_to_return.occurrence = max_amount_times_appears;
    }

    for (key, value) in hashmap_param
    {
        if *value == mode_to_return.occurrence
        {
            mode_to_return.vector_of_modes.push(*key);
        }
    }

    mode_to_return
}