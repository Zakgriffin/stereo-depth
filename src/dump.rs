// pub fn depth_sense_rayon_dots(
//     image_l: &Box<GrayImage>,
//     image_r: &Box<GrayImage>,
//     block_size: Block,
//     search_range: u32,
// ) -> Box<GrayImage> {
//     let (width, height) = image_l.dimensions();

//     let mut disparity_map_vec = vec![0u8; (width * height) as usize];

//     let cores = num_cpus::get();
//     let vec_len = disparity_map_vec.len();
//     let slice_length_max = vec_len / cores;

//     disparity_map_vec
//         .par_chunks_mut(slice_length_max)
//         .enumerate()
//         .for_each(|(n, slice)| {
//             let start = (slice.len() * n) as u32;
//             let mut x = ceil_to_nearest(start % width, block_size.width);
//             let mut y = ceil_to_nearest(start / width, block_size.height);

//             let start_next = (slice.len() * (n + 1)) as u32;
//             let x_end = floor_to_nearest(start_next % width, block_size.width);
//             let y_end = floor_to_nearest(start_next / width, block_size.height);

//             println!(
//                 "n: {} | x: {}, y: {} | x_end: {}, y_end: {}",
//                 n, x, y, x_end, y_end
//             );

//             'outer: while y <= height - block_size.height {
//                 while x <= width - block_size.width {
//                     if x >= x_end && y >= y_end {
//                         break 'outer;
//                     }
//                     let window_left = image_l.view(x, y, block_size.width, block_size.height);
//                     let mut best_dif = u32::MAX;
//                     let mut disparity_val = 0;
//                     for x_scan in (x..min(x + search_range, width - block_size.width)).step_by(2) {
//                         let window_right =
//                             image_r.view(x_scan, y, block_size.width, block_size.height);
//                         let dif = image_dif(&window_left, &window_right, &block_size);
//                         if dif < best_dif {
//                             best_dif = dif;
//                             disparity_val = x_scan as i32 - x as i32;
//                             if dif == 0 {
//                                 break;
//                             }
//                         }
//                     }
//                     if slice.len() < (x + (y * width) - start) as usize {
//                         println!(
//                             "{} + ({} * width)) [{}] as usize - {}",
//                             x,
//                             y,
//                             x + (y * width),
//                             start
//                         );
//                     }
//                     slice[(x + (y * width) - start) as usize] = disparity_val as u8;
//                     x += block_size.width;
//                 }
//                 y += block_size.height;
//                 x = 0;
//             }
//         });

//     Box::new(GrayImage::from_vec(width, height, disparity_map_vec).unwrap())
// }

// fn ceil_to_nearest(n: u32, to: u32) -> u32 {
//     if n % to == 0 {
//         n
//     } else {
//         (n / to + 1) * to
//     }
// }
// fn floor_to_nearest(n: u32, to: u32) -> u32 {
//     n / to * to
// }

// let image = source_index
// .par_chunks(chunksize)
// .zip(weight.par_chunks(chunksize))
// .map(|(index_chunk, weight_chunk)| {
//     let mut image = vec![0.0; size * size];
//     for (index, weight) in index_chunk.into_iter().zip(weight_chunk) {
//         let index = *index as usize;
//         let y1 = to_pixel(cell_positions[index * 3 + 1]);
//         let y2 = to_pixel(cell_positions[index * 3 + 1] + cell_size[index]);

//         let x1 = to_pixel(cell_positions[index * 3]);
//         let x2 = to_pixel(cell_positions[index * 3] + cell_size[index]);

//         let npix = (y2 - y1) * (x2 - x1);
//         if npix == 0 {
//             image[y1 * size + x1] += weight;
//         } else {
//             for y in y1..y2 {
//                 for x in x1..x2 {
//                     image[y * size + x] += weight / npix as f64;
//                 }
//             }
//         }
//     }
//     image
// })
// .reduce(
//     || vec![0.0; 1000 * 1000],
//     |mut image, subimage| {
//         for (i, si) in image.iter_mut().zip(&subimage) {
//             *i += *si
//         }
//         image
//     },
// );

// https://users.rust-lang.org/t/two-dimensional-iterating/27538/3
// fn gen_2d_range(from: u32, to: u32) -> impl Iterator<Item = (u32, u32)> {
//     let a = (from..to).into_par_iter();
//     (from..to).flat_map(move |a| (from..to).map(move |b| (a, b)))
// }
