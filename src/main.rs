extern crate image;

use image::{GenericImageView, ImageBuffer, RgbImage};
use std::cmp::{min,max};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn top(&self) -> i32 {
        self.y
    }

    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }


    // return if this rectagnle intersects another
    pub fn intersects(&self, other: &Rect) -> bool {
      return self.left() <= other.right() && other.left() <= self.right() &&
             self.top() <= other.bottom() && other.top() <= self.bottom();
    }

    /*
     * return the intersection of two Rects
     */
    pub fn intersection(a: &Rect, b: &Rect) -> Rect {
        let l = max(a.left(), b.left());
        let r = min(a.right(), b.right());
        let t = max(a.top(), b.top());
        let b = min(a.bottom(), b.bottom());

        // Do they actually intersect?
        if l > r || t > b {
            return Rect { x: 0, y: 0, w: 0, h: 0 };
        }

        return Rect::new(l, t, (r-l) + 1, (b - t) + 1);
    }
}



struct Screen {
    img: image::RgbImage,
    w: u32, h: u32,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            img: image::RgbImage::new(width, height),
            w: width, h: height
        }
    }

    pub fn draw_rect(&mut self, r: Rect, c: image::Rgb<u8>) {
        for y in r.top()..r.bottom() {
            for x in r.left()..r.right() {
                self.img.put_pixel(x as u32, y as u32, c);
            }
        }
    }

    pub fn compose(&mut self, file_name: &str, windows: Vec<(Rect, image::Rgb<u8>)>) {
        // clear screen
        self.draw_rect(Rect::new(0, 0, self.w as i32, self.h as i32), image::Rgb([0x33, 0x33, 0x33]));
        for (w, c) in &windows {
            self.draw_rect(*w, *c);
        }

        self.img.save(file_name).unwrap();
    }
}


fn main() {
    let mut screen = Screen::new(1024, 768);
    let mut windows = vec![];

    windows.push((Rect::new(220, 50, 500, 500), image::Rgb([0, 0, 255])));
    windows.push((Rect::new(400, 400, 300, 200), image::Rgb([0, 255, 0])));

    screen.compose("screen.png", windows);
}
