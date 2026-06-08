use b::step;

fn main() -> image::ImageResult<()> {
    b::display::print_file_header("examples/image_io/main.rs");

    step!("1. 创建 193x193 透明底图像，导出为 PNG", {
        let block_img = b::block_image::CubeBlockImage::default();
        block_img.save("examples/image_io/export/cube_transparent.png")?;
        [wait]
    });

    step!("2. 白色背景导出", {
        let white = b::color::RGBAColor {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        let block_img = b::block_image::CubeBlockImage::new(white);
        block_img.save("examples/image_io/export/cube_white.png")?;
        [wait]
    });

    step!("3. 加载 PNG 再重新保存（往返测试）", {
        let img = b::image::RGBAImage::<193, 193>::load("examples/image_io/export/cube_white.png")?;
        img.save("examples/image_io/export/cube_roundtrip.png")?;
        [wait]
    });

    Ok(())
}
