use vecmath::*;
use opengl_graphics::*;
use texture_cache::*;
use piston::event::*;
use graphics::Image;
use graphics::default_draw_state;

//todo: write a tilemap -> source rect converter, instead of directly converting TileStates into source rects

#[derive(Debug, Clone, Copy)]
pub enum TileState {
    Floor = 0,
    Wall = 1,
    Void = 2
}

/// A first-pass implementation of a tile map struct for rendering world levels.
pub struct TileMap {
    tile_states : Vec<Vec<TileState>>,
    tile_images : Vec<Vec<Image>>,
    tile_size : Vector2<i32>,
    tile_asset : String,
    tile_map_size : Vector2<i32>,
}

impl TileMap {
    pub fn with_default(tile_asset : String, tile_map_size : Vector2<i32>, tile_size : Vector2<i32>) -> TileMap {
        let mut tile_states : Vec<Vec<TileState>> = Vec::with_capacity(tile_map_size[1] as usize);
        let mut tile_images : Vec<Vec<Image>> = Vec::with_capacity(tile_map_size[1] as usize);

        for _ in 0..tile_map_size[1] {
            let mut tile_row_states : Vec<TileState> = Vec::with_capacity(tile_map_size[0] as usize);
            for _ in 0..tile_map_size[0] {
                tile_row_states.push(TileState::Floor);
            }
            tile_states.push(tile_row_states);
        }

        for y in 0..tile_map_size[1] {
            let mut tile_row_images : Vec<Image> = Vec::with_capacity(tile_map_size[0] as usize);
            for x in 0..tile_map_size[0] {
                let tile_image = Image::new()
                            .rect([(x * tile_size[0]) as f64, (y * tile_size[1]) as f64, tile_size[0] as f64, tile_size[1] as f64])
                            .src_rect(TileMap::get_src_rect(tile_size, tile_states[x as usize][y as usize]));
                tile_row_images.push(tile_image);
            }
            tile_images.push(tile_row_images);
        }

        TileMap {
            tile_states : tile_states,
            tile_size : tile_size,
            tile_images : tile_images,
            tile_asset : tile_asset,
            tile_map_size : tile_map_size,
        }
    }

    pub fn set_tile_state(&mut self, tile : Vector2<i32>, state : TileState) {
        if tile[0] < 0 || tile[1] < 0 {
            panic!("attempted to set tile {:?} to state {:?}, which is out of bounds negatively (tile map size is {:?})", tile, state, self.tile_map_size);
        }
        if tile[0] >= self.tile_map_size[0] || tile[1] >= self.tile_map_size[1] {
            panic!("attempted to set tile {:?} to state {:?}, which is out of bounds (tile map size is {:?})", tile, state, self.tile_map_size);
        }
        let x = tile[0] as usize;
        let y = tile[1] as usize;

        self.tile_states[x][y] = state;

        //when calling src_rect, the change creates a new copy of Image (or else the change is swallowed into the void) if you don't re-assign it to the original variable
        //Not sure why, but then I suck at rust ownership semantics at the moment
        self.tile_images[x][y] = self.tile_images[x][y].src_rect(TileMap::get_src_rect(self.tile_size, state));
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, tex_cache: &mut TextureCache) {
        let tilemap_tex = tex_cache.get_asset(&self.tile_asset);
        let tile_images = &self.tile_images;

        gl.draw(args.viewport(), |c, gl| {

                for row in tile_images {
                    for tile in row {
                        tile.draw(tilemap_tex, default_draw_state(), c.transform, gl);
                    }
                }
            });
    }

    fn get_src_rect(tile_size : Vector2<i32>, tile : TileState) -> [i32; 4] {
        [tile_size[0] * tile as i32, 0, tile_size[0], tile_size[1]]
    }
}
