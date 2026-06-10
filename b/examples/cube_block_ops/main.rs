use b::step;

fn main() -> image::ImageResult<()> {
    b::display::print_file_header("examples/cube_block_ops/main.rs");
    b::display::set_typewriter(true);

    step!("Cube Block Operations", {
        let cube = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let mut block = b::block::CubeBlock::new();

        step!("1. 创建空区块", {
            assert!(block.is_empty_block());
            println!("空区块创建成功，方块数: {}", block.count_occupied_slots());
            let img = block.render_to_rgba_image();
            img.save_as_png_file("examples/cube_block_ops/export/01_empty.png")?;
            img.show_self();
        });

        step!("2. 放置四个方块", {
            block.place_cube(0, 0, 0, Box::new(cube.clone()));
            block.place_cube(2, 0, 0, Box::new(cube.clone()));
            block.place_cube(0, 2, 0, Box::new(cube.clone()));
            block.place_cube(0, 0, 2, Box::new(cube.clone()));
            assert_eq!(block.count_occupied_slots(), 4);
            let img = block.render_to_rgba_image();
            img.save_as_png_file("examples/cube_block_ops/export/02_four_cubes.png")?;
            img.show_self();
        });

        step!("3. 移除一个方块", {
            block.remove_cube(0, 0, 2);
            assert_eq!(block.count_occupied_slots(), 3);
            let img = block.render_to_rgba_image();
            img.save_as_png_file("examples/cube_block_ops/export/03_three_cubes.png")?;
            img.show_self();
        });

        step!("4. 清空区块", {
            block.clear_all_cubes();
            assert!(block.is_empty_block());
            let img = block.render_to_rgba_image();
            img.save_as_png_file("examples/cube_block_ops/export/04_cleared.png")?;
            img.show_self();
        });
    });

    Ok(())
}
