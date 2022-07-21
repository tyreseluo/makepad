use {
    std::{
        sync::{Arc, Mutex},
        cell::RefCell,
    },
    crate::{
        makepad_platform::*,
        frame_component::*,
    }
};
// include the SIMD path if we support it
#[cfg(any(not(target_arch = "wasm32"), target_feature = "simd128"))]
use crate::mandelbrot_simd::*;

// Our live DSL to define the shader and UI def
live_register!{
    use makepad_platform::shader::std::*;
    
    // the shader to draw the texture tiles
    DrawTile: {{DrawTile}} {
        texture tex: texture2d
        fn pixel(self) -> vec4 {
            let fractal = sample2d(self.tex, vec2(self.pos.x, 1.0 - self.pos.y))
            
            // unpack iteration and magnitude squared from our u32 buffer
            let iter = fractal.y * 65535 + fractal.x * 255;
            let magsq = (fractal.w * 256 + fractal.z - 127);
            
            let index = abs((6.0 * iter / self.max_iter) - 0.1 * log(magsq));
            if iter > self.max_iter {
                return vec4(0, 0, 0, 1.0);
            }
            return vec4(Pal::iq2(index + self.color_cycle), 1.0);
        }
    }
    
    Mandelbrot: {{Mandelbrot}} {
        max_iter: 320,
    }
}

pub const TILE_SIZE_X: usize = 256;
pub const TILE_SIZE_Y: usize = 256;
pub const TILE_CACHE_SIZE: usize = 500;
pub const POOL_THREAD_COUNT: usize = 4;

// the shader struct used to draw

#[derive(Live, LiveHook)]
#[repr(C)]
pub struct DrawTile {
    // this shader structs inherits from the super class DrawQuad
    // the shader compiler allows a form of inheritance where you
    // define a 'draw_super' field, which projects all values in the chain
    // onto the 'self' property in the shader. This is useful to partially reuse shadercode.
    draw_super: DrawQuad,
    // max iterations of the mandelbrot fractal
    max_iter: f32,
    // a value that cycles the color in the palette (0..1)
    color_cycle: f32
}

// basic plain f64 loop, not called in SIMD mode. 
// Returns the iteration count when the loop goes to infinity,
// and the squared magnitude of the complex number at the time of exit
// you can use this number to create the nice color bands you see in the output
// For a more detailed description, see mandelbrot explanations online
#[allow(dead_code)]
fn mandelbrot_pixel_f64(max_iter: usize, c_x: f64, c_y: f64) -> (usize, f64) {
    let mut x = c_x;
    let mut y = c_y;
    let mut magsq = 0.0;
    for n in 0..max_iter {
        let xy = x * y;
        let xx = x * x;
        let yy = y * y;
        magsq = xx + yy;
        if magsq > 4.0 {
            return (n, magsq)
        }
        x = (xx - yy) + c_x;
        y = (xy + xy) + c_y;
    }
    return (max_iter, magsq)
}

#[allow(dead_code)]
fn mandelbrot_f64(tile: &mut Tile, max_iter: usize) {
    let tile_size = vec2f64(TILE_SIZE_X as f64, TILE_SIZE_Y as f64);
    for y in 0..TILE_SIZE_Y {
        for x in 0..TILE_SIZE_X {
            let fp = tile.fractal.pos + tile.fractal.size * (vec2f64(x as f64, y as f64) / tile_size);
            let (iter, dist) = mandelbrot_pixel_f64(max_iter, fp.x, fp.y);
            let dist = (dist * 256.0 + 127.0 * 255.0).max(0.0).min(65535.0) as u32;
            tile.buffer[y * TILE_SIZE_X + x] = iter as u32 | (dist << 16);
        }
    }
}

pub struct Tile {
    // the memory buffer thats used when a tile is rendered
    pub buffer: Vec<u32>,
    // the makepad system texture backing the tile, when ready for drawing 'buffer' is swapped onto it
    pub texture: Texture,
    // the fractal space rectangle that this tile represents
    pub fractal: RectF64,
}

impl Tile {
    fn new(cx: &mut Cx) -> Self {
        let texture = Texture::new(cx);
        texture.set_desc(cx, TextureDesc {
            format: TextureFormat::ImageBGRA,
            width: Some(TILE_SIZE_X),
            height: Some(TILE_SIZE_Y),
            multisample: None
        });
        // preallocate buffers otherwise safari barfs in the worker
        let mut buffer = Vec::new();
        buffer.resize(TILE_SIZE_X * TILE_SIZE_Y, 0);
        Self {
            buffer,
            texture,
            fractal: RectF64::default()
        }
    }
}

// used to last minute test if a tile is outside the view
#[derive(Clone, Default)]
pub struct BailWindow {
    space: RectF64, // fractal space window
    is_zoom_in: bool // used to determine if we are zooming in
}

pub struct TileCache {
    // the current layer of tiles
    current: Vec<Tile>,
    // next layer of tiles
    next: Vec<Tile>,
    // the empty tilecache we can render to from workers
    empty: Vec<Tile>,
    current_zoom: f64,
    next_zoom: f64,
    tiles_in_flight: usize,
    
    thread_pool: ThreadPool,
    bail_window: Arc<Mutex<RefCell<BailWindow >> >,
}

impl TileCache {
    fn new(cx: &mut Cx) -> Self {
        let mut empty = Vec::new();
        for _ in 0..TILE_CACHE_SIZE {
            empty.push(Tile::new(cx));
        }
        Self {
            current: Vec::new(),
            next: Vec::new(),
            empty,
            current_zoom: 0.0,
            next_zoom: 0.0,
            tiles_in_flight: 0,
            thread_pool: ThreadPool::new(cx, POOL_THREAD_COUNT),
            bail_window: Default::default(),
        }
    }
    
    fn tile_completed(&mut self, cx: &mut Cx, mut tile: Tile) {
        self.tiles_in_flight -= 1;
        tile.texture.swap_image_u32(cx, &mut tile.buffer);
        self.next.push(tile)
    }
    
    fn tile_bailed(&mut self, tile: Tile) {
        self.tiles_in_flight -= 1;
        self.empty.push(tile);
    }
    
    fn set_bail_window(&self, bail_window: BailWindow) {
        *self.bail_window.lock().unwrap().borrow_mut() = bail_window;
    }
    
    fn tile_needs_to_bail(tile: &Tile, bail_window: Arc<Mutex<RefCell<BailWindow >> >) -> bool {
        let bail = bail_window.lock().unwrap().borrow().clone();
        if bail.is_zoom_in {
            if !tile.fractal.intersects(bail.space) {
                return true
            }
        }
        else { // compare the size of the bail window against the tile
            if tile.fractal.size.x * tile.fractal.size.y < bail.space.size.x * bail.space.size.y * 0.007 {
                return true
            }
        }
        false
    }
    
    fn generate_completed(&self) -> bool {
        self.tiles_in_flight == 0
    }
    
    fn discard_next_layer(&mut self, cx: &mut Cx) {
        while let Some(mut tile) = self.next.pop() {
            tile.texture.swap_image_u32(cx, &mut tile.buffer);
            self.empty.push(tile);
        }
    }
    
    fn discard_current_layer(&mut self, cx: &mut Cx) {
        while let Some(mut tile) = self.current.pop() {
            tile.texture.swap_image_u32(cx, &mut tile.buffer);
            self.empty.push(tile);
        }
        self.current_zoom = self.next_zoom;
        std::mem::swap(&mut self.current, &mut self.next);
    }
    
    // generates a queue
    pub fn generate_tasks_and_flip_layers(&mut self, cx: &mut Cx, zoom: f64, center: Vec2F64, window: RectF64, is_zoom_in: bool) -> Vec<Tile> {
        let size = vec2f64(zoom, zoom);
        
        // discard the next layer if we don't fill the screen yet at this point and reuse old
        if is_zoom_in && !self.next.is_empty() && self.next[0].fractal.size.x < 0.8 * zoom {
            self.discard_next_layer(cx);
        }
        else {
            self.discard_current_layer(cx);
        }
        
        self.next_zoom = zoom;
        
        let mut render_tasks = Vec::new();
        let window = window.add_margin(size);
        // create a spiralling walk around the center point, usually your mouse
        // this is a nice pattern because you look at and zoom around your mouse
        // and so rendering those tiles first in a circular pattern is good UX
        Self::spiral_walk( | _step, i, j | {
            let fractal = RectF64 {
                pos: center + size * vec2f64(i as f64, j as f64) - 0.5 * size,
                size: size
            };
            if window.intersects(fractal) {
                if let Some(mut tile) = self.empty.pop() {
                    tile.fractal = fractal;
                    render_tasks.push(tile);
                }
                true
            }
            else {
                false
            }
        });
        self.tiles_in_flight = render_tasks.len();
        render_tasks
    }
    
    // creates a nice spiral ordering to the tile rendering
    fn spiral_walk<F: FnMut(usize, isize, isize) -> bool>(mut f: F) {
        let mut di = 1;
        let mut dj = 0;
        let mut seg_len = 1;
        let mut i = 0;
        let mut j = 0;
        let mut seg_pass = 0;
        let mut any_intersect = false;
        let mut intersect_step = 0;
        for step in 0..100000 {
            if f(step, i, j) {
                any_intersect = true;
            }
            i += di;
            j += dj;
            seg_pass += 1;
            if seg_len == seg_pass {
                seg_pass = 0;
                let t = di;
                di = -dj;
                dj = t;
                if dj == 0 {
                    intersect_step += 1;
                    // cover the case that a spiral-edge step up does not match
                    // a complete circle
                    if intersect_step > 2 {
                        // at the end of a circular walk
                        // we check if we had any intersections with the viewport.
                        // (the closure returned true)
                        // ifso we keep spiralling
                        // otherwise we are done
                        if !any_intersect {
                            return
                        }
                        intersect_step = 0;
                        any_intersect = false;
                    }
                    seg_len += 1;
                }
            }
        }
    }
}

pub enum ToUI {
    TileDone {tile: Tile},
    TileBailed {tile: Tile},
}

// Space transforms from view (screen) to fractal and back
#[derive(Default, Clone)]
pub struct FractalSpace {
    // the rectangle of the viewport on screen
    view_rect: Rect,
    // the size of the tile in fractal space
    tile_size: Vec2F64,
    // the center of the fractal space
    center: Vec2F64,
    // the zoomfactor in the fractal space
    zoom: f64,
}

impl FractalSpace {
    fn new(center: Vec2F64, zoom: f64) -> Self {
        Self {
            center,
            zoom,
            ..Self::default()
        }
    }
    
    // constructs a copy of self with other zoom/center values
    fn other(&self, other_zoom: f64, other_center: Vec2F64) -> Self {
        Self {
            center: other_center,
            zoom: other_zoom,
            ..self.clone()
        }
    }
    
    fn fractal_to_screen(&self, pos: Vec2F64) -> Vec2 {
        let view_center = self.view_rect.pos + self.view_rect.size * 0.5;
        return (((pos - self.center) / self.zoom) * self.tile_size).into_vec2() + view_center;
    }
    
    fn screen_to_fractal(&self, pos: Vec2) -> Vec2F64 {
        let view_center = self.view_rect.pos + self.view_rect.size * 0.5;
        return (((pos - view_center).into_vec2f64() / self.tile_size) * self.zoom) + self.center;
    }
    
    fn fractal_to_screen_rect(&self, rect: RectF64) -> Rect {
        let pos1 = self.fractal_to_screen(rect.pos);
        let pos2 = self.fractal_to_screen(rect.pos + rect.size);
        Rect {
            pos: pos1,
            size: pos2 - pos1
        }
    }
    
    // transform a rect in view space to fractal space
    fn screen_to_fractal_rect(&self, rect: Rect) -> RectF64 {
        let pos1 = self.screen_to_fractal(rect.pos);
        let pos2 = self.screen_to_fractal(rect.pos + rect.size);
        RectF64 {
            pos: pos1,
            size: pos2 - pos1
        }
    }
    
    // this zooms the fractal space around a point on the screen
    fn zoom_around(&mut self, factor: f64, around: Vec2) {
        // hold on to the current position in fractal space
        let fpos1 = self.screen_to_fractal(around);
        self.zoom *= factor;
        if self.zoom < 5e-14f64 { // maximum zoom for f64
            self.zoom = 5e-14f64
        }
        if self.zoom > 2.0 { // don't go too far out
            self.zoom = 2.0;
        }
        let fpos2 = self.screen_to_fractal(around);
        // by comparing the position in fractal space before and after the zoomstep
        // we can move the center so it stays in the same spot
        self.center += fpos1 - fpos2;
    }
    
    // self.view_rect in fractal space
    fn view_rect_to_fractal(&self) -> RectF64 {
        self.screen_to_fractal_rect(self.view_rect)
    }
}


#[derive(Live, FrameComponent)]
#[live_register(frame_component!(Mandelbrot))]
pub struct Mandelbrot {
    // DSL accessible
    draw_tile: DrawTile,
    max_iter: usize,
    // thew view container that contains our mandelbrot UI
    view: View,
    // the 'walk' of the mandelbrot view, used in layouting
    walk: Walk,
    
    // prepending #[rust] makes derive(Live) ignore these fields
    // and they dont get DSL accessors
    #[rust] next_frame: NextFrame,
    // where your finger/mouse was when moved
    #[rust] finger_abs: Vec2,
    // set to true when the fractal is actively zoom animating
    #[rust] is_zooming: bool,
    
    // this bool flips wether or not you were zooming in or out
    // used to decide tile generation strategy
    #[rust(true)]
    is_zoom_in: bool,
    
    // default fractal space for looking at a mandelbrot
    #[rust(FractalSpace::new(vec2f64(-0.5, 0.0), 0.5))]
    space: FractalSpace,
    
    // the tilecache holding all the tiles
    #[rust(TileCache::new(cx))]
    tile_cache: TileCache,
    
    // the channel that can transmit events to the UI from workers
    #[rust] to_ui: ToUIReceiver<ToUI>,
}

impl LiveHook for Mandelbrot {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // starts the animation cycle on startup
        self.next_frame = cx.new_next_frame();
    }
}

#[derive(Clone, FrameComponentAction)]
pub enum MandelbrotAction {
    None
}

impl Mandelbrot {
    
    // the SIMD tile rendering, uses the threadpool to draw the tile
    #[cfg(any(not(target_arch = "wasm32"), target_feature = "simd128"))]
    pub fn render_tile(&mut self, mut tile: Tile, fractal_zoom: f64) {
        // lets swap our texture to the tile
        let max_iter = self.max_iter;
        let to_ui = self.to_ui.sender();
        let bail_window = self.tile_cache.bail_window.clone();
        self.tile_cache.thread_pool.execute(move || {
            if TileCache::tile_needs_to_bail(&tile, bail_window) {
                return to_ui.send(ToUI::TileBailed {tile}).unwrap();
            }
            if fractal_zoom >2e-5 {
                mandelbrot_f32_simd(&mut tile, max_iter);
            }
            else {
                mandelbrot_f64_simd(&mut tile, max_iter);
            }
            to_ui.send(ToUI::TileDone {tile}).unwrap();
        })
    }
    
    // Normal tile rendering, uses the threadpool to draw the tile
    #[cfg(all(target_arch = "wasm32", not(target_feature = "simd128")))]
    pub fn render_tile(&mut self, mut tile: Tile, _fractal_zoom: f64) {
        // lets swap our texture to the tile
        let max_iter = self.max_iter;
        let to_ui = self.to_ui.sender();
        let bail_window = self.tile_cache.bail_window.clone();
        self.tile_cache.thread_pool.execute(move || {
            if TileCache::tile_needs_to_bail(&tile, bail_window) {
                return to_ui.send(ToUI::TileBailed {tile}).unwrap();
            }
            mandelbrot_f64(&mut tile, max_iter);
            to_ui.send(ToUI::TileDone {tile}).unwrap();
        })
    }
    
    pub fn generate_tiles_around_finger(&mut self, cx: &mut Cx, zoom: f64, finger: Vec2) {
        self.generate_tiles(
            cx,
            zoom,
            self.space.other(zoom, self.space.center).screen_to_fractal(finger),
            self.space.other(zoom, self.space.center).view_rect_to_fractal(),
            self.is_zoom_in
        );
    }
    
    // generates the tiles and emits them in the right spiral order
    pub fn generate_tiles(&mut self, cx: &mut Cx, zoom: f64, center: Vec2F64, window: RectF64, is_zoom_in: bool) {
        let render_tasks = self.tile_cache.generate_tasks_and_flip_layers(cx, zoom, center, window, is_zoom_in);
        if is_zoom_in {
            for tile in render_tasks {
                self.render_tile(tile, zoom)
            }
        }
        else { // on zoom out reverse the spiral compared to zoom_in
            for tile in render_tasks.into_iter().rev() {
                self.render_tile(tile, zoom)
            }
        }
    }
    
    pub fn handle_event(&mut self, cx: &mut Cx, event: &mut Event) -> MandelbrotAction {
        //self.state_handle_event(cx, event);
        
        if let Event::Signal(_) = event {
            // this batches up all the input signals into a single animation frame
            self.next_frame = cx.new_next_frame();
        }
        
        if let Some(ne) = self.next_frame.triggered(event) {
            let mut tiles_received = 0;
            // try pulling tiles from our message channel from the worker threads
            while let Ok(msg) = self.to_ui.receiver.try_recv() {
                match msg {
                    ToUI::TileDone {tile} => {
                        self.tile_cache.tile_completed(cx, tile);
                        
                        // trigger a new tile render if we didn't render pixel accurate already
                        if self.tile_cache.generate_completed() && self.tile_cache.next_zoom != self.space.zoom {
                            let zoom = self.space.zoom * if self.is_zooming {if self.is_zoom_in {0.8}else {2.0}}else {1.0};
                            self.generate_tiles_around_finger(cx, zoom, self.finger_abs);
                        }
                        
                        tiles_received += 1;
                        // dont process too many tiles at once as this hiccups the renderer
                        if tiles_received > 10 {
                            break;
                        }
                    }
                    ToUI::TileBailed {tile} => {
                        self.tile_cache.tile_bailed(tile);
                    }
                }
            }
            // initial tile render
            if self.tile_cache.generate_completed() && self.tile_cache.current.is_empty() {
                self.generate_tiles_around_finger(cx, self.space.zoom, self.space.view_rect.center());
            }
            
            if self.is_zooming { // this only fires once the zoom is starting and the queue is emptying
                self.space.zoom_around(if self.is_zoom_in {0.98} else {1.02}, self.finger_abs);
                // this kickstarts the tile cache generation when zooming.
                if self.tile_cache.generate_completed() {
                    let zoom = self.space.zoom * if self.is_zoom_in {0.8} else {2.0};
                    self.generate_tiles_around_finger(cx, zoom, self.finger_abs);
                }
            }
            
            // animnate color cycle
            self.draw_tile.color_cycle  = (ne.time * 0.2).fract() as f32;
            
            // this triggers a draw_walk call
            self.view.redraw(cx);
            self.next_frame = cx.new_next_frame();
        }
        
        match event.hits_with_options(cx, self.view.area(), HitOptions {use_multi_touch: true, margin: None}) {
            HitEvent::FingerDown(fe) => {
                self.is_zooming = true;
                if !fe.input_type.is_touch() || fe.digit == 0 {
                    self.finger_abs = fe.abs;
                }
                if fe.digit == 0 {
                    self.is_zoom_in = true;
                }
                else {
                    self.is_zoom_in = false;
                }
                
                self.view.redraw(cx);
                
                self.next_frame = cx.new_next_frame();
            },
            HitEvent::FingerMove(fe) => {
                if !fe.input_type.is_touch() || fe.digit == 0 {
                    self.finger_abs = fe.abs;
                }
            }
            HitEvent::FingerUp(fe) => {
                if fe.input_type.is_touch() && fe.digit == 1 {
                    self.is_zoom_in = true;
                }
                else {
                    self.is_zoom_in = true;
                    self.is_zooming = false;
                }
            }
            _ => ()
        }
        MandelbrotAction::None
    }
    
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) -> ViewRedraw {
        self.view.begin(cx, walk, Layout::flow_right()) ?;
        
        // store the view information here as its the only place it's known in the codeflow
        self.space.tile_size = vec2f64(TILE_SIZE_X as f64, TILE_SIZE_Y as f64) / cx.current_dpi_factor as f64;
        self.space.view_rect = cx.turtle().rect();
        
        // update bail window the workers check to skip tiles that are no longer in view
        self.tile_cache.set_bail_window(BailWindow {
            is_zoom_in: self.is_zoom_in,
            space: self.space.view_rect_to_fractal()
        });
        
        // pass the data onto the shader object
        self.draw_tile.max_iter = self.max_iter as f32;
                // iterate the current and next tile caches and draw the fractal tile
        for tile in self.tile_cache.current.iter().chain(self.tile_cache.next.iter()) {
            let rect = self.space.fractal_to_screen_rect(tile.fractal);
            self.draw_tile.draw_vars.set_texture(0, &tile.texture);
            self.draw_tile.draw_abs(cx, rect);
        }
        
        self.view.end(cx);
        
        Ok(())
    }
}