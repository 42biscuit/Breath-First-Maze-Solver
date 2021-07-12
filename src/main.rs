use core::time;
use std::{borrow::Borrow, u32};

use image::save_buffer;

fn main() {
    let mut path = String::from("C://Users//bigey//OneDrive//Desktop//map3.bmp");
    let data_00 = image::open(&mut path).unwrap().as_bytes().to_vec();
    let data_0 = clean_arr(data_00.clone());
    let mut visited = Vec::new();
    let mut route = Vec::new();
    let (map_x, map_y) = image::image_dimensions(path).unwrap() ;
    let (start,end) = find_end_start(&data_0, map_x, map_y);
    route.push(start+map_x);
    visited.push(start+map_x);

    'end_found:loop{
        let best_point = get_options(&data_0, route.last().unwrap().to_owned(), map_x, map_y, &visited, end);
        if best_point == 0{
            route.pop();

        }else{
            if best_point == end-map_x {
                println!("end is found   {:?}",&route);
                fill_nodes_colour(route, data_00, map_x, map_y);
                break 'end_found;
            }
            visited.push(best_point);
            route.push(best_point);
            println!("route {:?}",&route);
        }
        
    }
}
fn get_options(data_0: &Vec<u8>,current:u32,map_x: u32,map_y: u32,visited:&Vec<u32>,end: u32)->u32{
    let mut best = (map_x*map_x) as f32;
    let mut point= map_x*map_x;
    if data_0[current as usize + 1] == 255 && visited.contains((current  + 1).borrow()) == false {
        if get_f(end, map_x, current + 1) < best{
            best = get_f(end, map_x, current + 1);
            point = current + 1;
        }
    }
    if data_0[current as usize - 1] == 255  &&  visited.contains((current - 1).borrow()) == false{
        if get_f(end, map_x, current - 1) < best{
            best = get_f(end, map_x, current - 1);
            point = current - 1;
        }
    }
    if data_0[current as usize + map_x as usize ] == 255 &&  visited.contains((current  + map_x ).borrow()) == false {
        if get_f(end, map_x, current + map_x) < best{
            best = get_f(end, map_x, current + map_x);
            point = current + map_x;
        }
    }
    if data_0[current as usize - map_x as usize ] == 255 &&  visited.contains((current -map_x).borrow()) == false {
        if get_f(end, map_x, current - map_x) < best{
            best = get_f(end, map_x, current - map_x);
            point = current - map_x;
        }
    }
    if best ==(map_x*map_x)as f32{
        point = 0;//ie there are not  solutions
    }
    point

}
fn get_f(end: u32, map_x: u32, point: u32) -> f32 {
    // this gives us a distance of the point that we are looking at to the end as the bird flies.
    /*let end_row = end /map_x;
    let end_colmn = end %map_x; // i ahev left these variables here to make the distance easier to read, all it does is use puthagoruses theorem to find the distance
    let point_row = point/map_x;
    let point_colmn = point %map_x;*/
    println!("point: {}",point);
    let dist = ((((point-point%map_x)as i32 -(end-point%map_x)as i32).pow(2)+((point%map_x)as i32-(point%map_x)as i32).pow(2)) as f32).powf(0.5);

        //finally it squiare roots it
    println!("{}",dist);
    dist
}


fn clean_arr(mut data: Vec<u8>) -> Vec<u8> {
    let mut itera = 0;
    while itera != data.len() {
        // only keep every 3rd value as that is enough to tell us if it is black or wgiht
        data.remove(itera);
        data.remove(itera);
        itera += 1;
    }
    data
}

fn find_end_start(data: &Vec<u8>, map_x: u32, map_y: u32) -> (u32, u32) {
    let mut start = 0;
    let mut end = 0;
    'start_found: for i in 0..map_x {
        //find the start on the first line (i.ethe only white pixel on the first line)
        if data[i as usize] == 255 {
            start = i;
            break 'start_found;
        }
    }
    'end_found: for i in 0..map_x {
        //find the end on the last line
        if data[(i + map_x * (map_y - 1)) as usize] == 255 {
            end = i + map_x * (map_y - 1);
            break 'end_found;
        }
    }
    (start, end)
}
fn fill_nodes_colour(route: Vec<u32>, mut data_0: Vec<u8>, map_x: u32, map_y: u32) {
    println!("{:?}", route);
    for i in route {
        data_0[i as usize * 3] = 225;
        data_0[(i as usize * 3) + 1] = 0;
        data_0[(i as usize * 3) + 2] = 0;
    }

    //println!("{:?}",data_0);

    let colour = image::ColorType::Rgb8;
    match save_buffer(
        "C://Users//bigey//OneDrive//Desktop//map_solution.bmp",
        &mut data_0.clone(),
        map_x,
        map_y,
        colour,
    ) {
        Ok(()) => {
            println!("it should be saved");
        }
        Err(e) => {
            //println!("{:?}", e);
        }
    }
}