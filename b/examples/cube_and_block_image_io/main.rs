use b::{cube::Cube, step};

fn main() -> image::ImageResult<()> {
    b::display::print_file_header("examples/cube_and_block_image_io/main.rs");

    step!("1. 加载方块图像并显示", {
        let cube =
            b::solid_cube::SolidCube::new("examples/cube_and_block_image_io/assets/some_cube.png")?;
        cube.show();
        [wait]
    });

    step!("2. 创建 193x193 透明底区块图像，导出为 PNG", {
        let block_img = b::block_image::CubeBlockImage::default();
        block_img.save("examples/cube_and_block_image_io/export/block_transparent.png")?;
        [wait]
    });

    step!("3. 白色背景区块图像导出", {
        let white = b::color::RGBAColor {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        let block_img = b::block_image::CubeBlockImage::new(white);
        block_img.save("examples/cube_and_block_image_io/export/block_white.png")?;
        [wait]
    });

    step!("4. 加载方块图像再重新保存（往返测试）", {
        let cube_img = b::image::RGBAImage::<13, 13>::load(
            "examples/cube_and_block_image_io/assets/some_cube.png",
        )?;
        cube_img.save("examples/cube_and_block_image_io/export/cube_roundtrip.png")?;
        [wait]
    });

    step!(
        "5. 加载 193x193 区块图像再重新保存（往返测试）",
        {
            let block_img = b::image::RGBAImage::<193, 193>::load(
                "examples/cube_and_block_image_io/export/block_white.png",
            )?;
            block_img.save("examples/cube_and_block_image_io/export/block_roundtrip.png")?;
            [wait]
        }
    );

    Ok(())
}
