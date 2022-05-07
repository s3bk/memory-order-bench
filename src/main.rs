pub struct BigImage {
    data: Vec<u32> // 8192 * 8192
}

#[derive(Copy, Clone)]
pub struct Tile ([[u32; 16]; 16]);
impl Tile {
    const fn zero() -> Self {
        Tile([[0; 16]; 16])
    }
}

impl BigImage {
    fn tiles(&self) -> Vec<Tile> {
        assert_eq!(self.data.len(), 8192 * 8192);
        let mut tiles = vec![Tile::zero(); 512 * 512];
        self.copy_to_tiles(&mut tiles);
        tiles
    }
    fn copy_to_tiles(&self, tiles: &mut Vec<Tile>) {
        assert_eq!(self.data.len(), 8192 * 8192);
        assert_eq!(tiles.len(), 512 * 512);
        for (input_chunk, ouput_chunk) in self.data.chunks_exact(16 * 8192).zip(tiles.chunks_exact_mut(512)) {
            for (t_y, input_row) in input_chunk.chunks_exact(8192).enumerate() {
                for (input_slice, tile) in input_row.chunks_exact(16).zip(ouput_chunk.iter_mut()) {
                    tile.0[t_y].copy_from_slice(input_slice);
                }
            }
        }
    }
    fn copy_to_tiles2(&self, tiles: &mut Vec<Tile>) {
        assert_eq!(self.data.len(), 8192 * 8192);
        assert_eq!(tiles.len(), 512 * 512);
        for (input_chunk, ouput_chunk) in self.data.chunks_exact(16 * 8192).zip(tiles.chunks_exact_mut(512)) {
            for (t_x, tile) in ouput_chunk.iter_mut().enumerate() {
                for (t_y, row) in tile.0.iter_mut().enumerate() {
                    let off = t_x * 16 + t_y * 8192;
                    let input = &input_chunk[off..off+16];
                    row.copy_from_slice(input);
                }
            }
        }
    }
}

fn main() {
    use benchmark_simple::*;

    let src = BigImage {
        data: vec![1; 8192 * 8192]
    };

    let mut tiles = vec![Tile::zero(); 512 * 512];
    
    
    let bench = Bench::new();
    let options = Options::default();
    let res1 = bench.run(&options, || src.copy_to_tiles(&mut tiles));
    println!("input order: {} GB/s", (src.data.len() * 4) as f64 / (res1.as_secs_f64() * (1024 * 1024 * 1024) as f64));
    
    let res2 = bench.run(&options, || src.copy_to_tiles2(&mut tiles));
    println!("output order: {} GB/s", (src.data.len() * 4) as f64 / (res2.as_secs_f64() * (1024 * 1024 * 1024) as f64));
    
}
