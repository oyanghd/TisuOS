//! 输入缓冲
//! 将所有输入存放进输入循环队列中
//! 输入的添加和取出使用两个互不干扰的变量，在 QUEUE_SIZE 长度的队列中循环存取
//! 
//! 2021年1月1日 zg

use core::ops::Sub;


const QUEUE_SIZE : usize = 128;
static mut MOUSE_CUR_IDX : usize = 0;
// static mut MOUSE_GET_IDX : usize = 0;
static mut KEY_PRESS_CUR_IDX : usize = 0;
static mut KEY_PRESS_GET_IDX : usize = 0;
pub static mut KEY_RELEASE_CUR_IDX : usize = 0;
pub static mut KEY_RELEASE_GET_IDX : usize = 0;
static mut MOUSE_POSITION : [Point;QUEUE_SIZE] = [Point::new();QUEUE_SIZE];
static mut KEY_PRESSED : [u16;QUEUE_SIZE] = [0;QUEUE_SIZE];
static mut KEY_RELEASE : [u16;QUEUE_SIZE] = [0;QUEUE_SIZE];

pub fn get_mouse_position()->Point{
    unsafe {
        // let idx = MOUSE_GET_IDX;
        // if MOUSE_CUR_IDX != MOUSE_GET_IDX{
        //     MOUSE_GET_IDX = (MOUSE_GET_IDX + 1) % QUEUE_SIZE;
        // }
        MOUSE_POSITION[(MOUSE_CUR_IDX + QUEUE_SIZE - 1) % QUEUE_SIZE]
    }
}

pub fn add_mouse_position(point : Point){
    unsafe {
        MOUSE_POSITION[MOUSE_CUR_IDX] = point;
        MOUSE_CUR_IDX = (MOUSE_CUR_IDX + 1) % QUEUE_SIZE;
    }
}

pub fn get_key_press()->u16{
    unsafe {
        let idx = KEY_PRESS_GET_IDX;
        if KEY_PRESS_GET_IDX != KEY_PRESS_CUR_IDX{
            KEY_PRESS_GET_IDX = (KEY_PRESS_GET_IDX + 1) % QUEUE_SIZE;
        }
        else{
            return 0;
        }
        KEY_PRESSED[idx]
    }
}

pub fn add_key_press(v : u16){
    unsafe {
        KEY_PRESSED[KEY_PRESS_CUR_IDX] = v;
        KEY_PRESS_CUR_IDX = (KEY_PRESS_CUR_IDX + 1) % QUEUE_SIZE;
    }
}

pub fn get_key_release()->u16{
    unsafe {
        let idx = KEY_RELEASE_GET_IDX;
        if KEY_RELEASE_GET_IDX != KEY_RELEASE_CUR_IDX{
            KEY_RELEASE_GET_IDX = (KEY_RELEASE_GET_IDX + 1) % QUEUE_SIZE;
        }
        else {
            return 0;
        }
        KEY_RELEASE[idx]
    }
}

pub fn add_key_release(v : u16){
    unsafe {
        KEY_RELEASE[KEY_RELEASE_CUR_IDX] = v;
        KEY_RELEASE_CUR_IDX = (KEY_RELEASE_CUR_IDX + 1) % QUEUE_SIZE;
    }
}

/// ## 坐标点
/// 0 ~ 1，表示在屏幕中的位置比例
#[derive(Clone, Copy, Debug)]
pub struct Point{
    pub x : f32,
    pub y : f32,
}
impl Point {
    pub const fn new()->Self{
        Self{
            x : 0.0,
            y : 0.0,
        }
    }
}

impl PartialEq for Point{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == self.y
    }
}

impl Sub for Point{
    type Output = Self;
    fn sub(self, rhs: Self) ->Self {
        Self{
            x : rhs.x - self.x,
            y : rhs.y - self.y,
        }
    }
}

// use crate::uart;
