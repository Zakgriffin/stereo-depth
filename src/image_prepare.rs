// use crate::Block;
// use image::GrayImage;

// pub struct ImagePrepare {
//     image_l: Box<GrayImage>,
//     image_r: Box<GrayImage>,
//     block_list: Vec<Block>,
//     max_block: u32,
//     min_block: u32,
//     width_block: Vec<u32>,
//     height_block: Vec<u32>,
// }

// impl ImagePrepare {
//     pub fn new(i_l: Box<GrayImage>, i_r: Box<GrayImage>, max_block: u32, min_block: u32) -> Self {
//         ImagePrepare {
//             image_l: i_l,
//             image_r: i_r,
//             block_list: vec![],
//             max_block: max_block,
//             min_block: min_block,
//             width_block: vec![],
//             height_block: vec![],
//         }
//     }

//     // checks if dimension value is prime or not
//     // fn is_prime(&self, x: u32) -> bool {
//     //     for i in self.min_block..self.max_block + 1 {
//     //         // 14
//     //         if x % i == 0 {
//     //             return false;
//     //         }
//     //     }
//     //     true
//     // }

//     // sets a new divisible size if dimension is a prime number
//     fn set_new_size(&mut self) {
//         // if self.is_prime(self.image_r.height()) {
//         //     let new_width = self.image_r.height() - 1;
//         //     let height = self.image_r.height();
//         //     self.image_l = crop(&mut self.image_l, 0, 0, new_width, height).to_image();
//         //     self.image_r = crop(&mut self.image_r, 0, 0, new_width, height).to_image();
//         // }
//         // if self.is_prime(self.image_r.width()) {
//         //     let new_height = self.image_r.width() - 1;
//         //     let width = self.image_r.width();
//         //     self.image_l = crop(&mut self.image_l, 0, 0, width, new_height).to_image();
//         //     self.image_r = crop(&mut self.image_r, 0, 0, width, new_height).to_image();
//         // }
//     }

//     // checks if right and left image dimensions are equal
//     fn check_shape(&self) -> bool {
//         self.image_l.dimensions() == self.image_r.dimensions()
//     }

//     // generates the block to optimise the execution time and accuracy of result
//     fn block_dimension(&mut self) {
//         self.width_block = vec![];
//         self.height_block = vec![];
//         // finding a common factor as length for tile
//         for i in (self.max_block..self.min_block - 1).rev() {
//             // 14
//             if self.image_l.width() % i == 0 {
//                 self.width_block.push(i);
//             }
//         }
//         // finding a common factor as height for tile
//         for i in (self.max_block..self.min_block - 1).rev() {
//             if self.image_l.height() % i == 0 {
//                 self.height_block.push(i);
//             }
//         }
//     }

//     // list a set of blocks for optimal shape
//     fn generate_block_list(&mut self) {
//         self.block_dimension();
//         for i in 0..self.width_block.len() {
//             for j in 0..self.height_block.len() {
//                 self.block_list.push(Block {
//                     width: self.width_block[i],
//                     height: self.height_block[j],
//                 });
//                 // if self.block_list.len() >= 1 {
//                 //     return self.block_list;
//                 // }
//                 return;
//                 // TODO what is happening with commented return? ^
//             }
//         }
//         // self.block_list
//     }

//     // iterates size modification until optimum size is reached
//     fn check_blocks(&mut self) -> u32 {
//         let mut count = 0;
//         while self.block_list.len() < 1
//             || self.width_block.len() == 0
//             || self.height_block.len() == 0
//         {
//             println!("yep");
//             count += 1;
//             self.block_list = vec![];
//             self.set_new_size();
//             self.generate_block_list();
//         }
//         count
//     }

//     // final call function
//     pub fn main(mut self) -> (Option<Vec<Block>>, (Box<GrayImage>, Box<GrayImage>)) {
//         if self.check_shape() {
//             if self.check_blocks() <= 60 {
//                 (Some(self.block_list), (self.image_l, self.image_r))
//             } else {
//                 (None, (self.image_l, self.image_r))
//             }
//         } else {
//             (Some(vec![]), (self.image_l, self.image_r))
//         }
//         // ew fucking gross I'm sorry
//     }
// }
