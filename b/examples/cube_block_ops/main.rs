use b::step;

fn main() -> image::ImageResult<()> {
    b::display::print_file_header("examples/cube_block_ops/main.rs");

    step!("1. 创建空区块，渲染并查看", {
        let block = b::block::CubeBlock::default();
        assert!(block.is_empty_block());
        println!("空区块创建成功，方块数: {}", block.count_occupied_slots());

        let block_img = block.render_to_rgba_image();
        block_img.save_as_png_file("examples/cube_block_ops/export/01_empty.png")?;
        [wait];

        block_img.show_self();
        [wait]
    });

    step!("2. 放置四个方块，渲染并查看", {
        let mut block = b::block::CubeBlock::default();

        let cube1 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let cube2 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let cube3 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let cube4 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;

        block.place_cube(0, 0, 0, Box::new(cube1));
        block.place_cube(2, 0, 0, Box::new(cube2));
        block.place_cube(0, 2, 0, Box::new(cube3));
        block.place_cube(0, 0, 2, Box::new(cube4));

        assert_eq!(block.count_occupied_slots(), 4);
        println!("四个方块放置成功，方块数: {}", block.count_occupied_slots());

        let block_img = block.render_to_rgba_image();
        block_img.save_as_png_file("examples/cube_block_ops/export/02_four_cubes.png")?;
        [wait];

        block_img.show_self();
        [wait]
    });

    step!("3. 移除一个方块，渲染并查看", {
        let mut block = b::block::CubeBlock::default();

        let cube1 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let cube2 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let cube3 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let cube4 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;

        block.place_cube(0, 0, 0, Box::new(cube1));
        block.place_cube(2, 0, 0, Box::new(cube2));
        block.place_cube(0, 2, 0, Box::new(cube3));
        block.place_cube(0, 0, 2, Box::new(cube4));

        let removed = block.remove_cube(0, 0, 2);
        assert!(removed.is_some());
        assert_eq!(block.count_occupied_slots(), 3);
        println!(
            "移除 (0,0,2) 成功，剩余方块数: {}",
            block.count_occupied_slots()
        );

        let block_img = block.render_to_rgba_image();
        block_img.save_as_png_file("examples/cube_block_ops/export/03_three_cubes.png")?;
        [wait];

        block_img.show_self();
        [wait]
    });

    step!("4. 清空区块，渲染并查看", {
        let mut block = b::block::CubeBlock::default();

        let cube1 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let cube2 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let cube3 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;
        let cube4 = b::solid_cube::SolidCube::new("examples/cube_block_ops/assets/some_cube.png")?;

        block.place_cube(0, 0, 0, Box::new(cube1));
        block.place_cube(2, 0, 0, Box::new(cube2));
        block.place_cube(0, 2, 0, Box::new(cube3));
        block.place_cube(0, 0, 2, Box::new(cube4));

        block.clear_all_cubes();
        assert_eq!(block.count_occupied_slots(), 0);
        assert!(block.is_empty_block());
        println!("清空成功，方块数: {}", block.count_occupied_slots());

        let block_img = block.render_to_rgba_image();
        block_img.save_as_png_file("examples/cube_block_ops/export/04_cleared.png")?;
        [wait];

        block_img.show_self();
        [wait]
    });

    Ok(())
}
