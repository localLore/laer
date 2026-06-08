use b::step;

fn main() -> image::ImageResult<()> {
    b::display::print_file_header("examples/export_cube/main.rs");

    step!("1. 创建 49×49 透明底图像，导出为 PNG", {
        let block_img = b::concept::CubeBlockImage::default();
        block_img.save("examples/export_cube/export/cube_transparent.png")?;
        [wait]
    });

    step!("2. 白色背景导出", {
        let white = b::data::RGBAColor {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        let block_img = b::concept::CubeBlockImage::new(white);
        block_img.save("examples/export_cube/export/cube_white.png")?;
        [wait]
    });

    step!("3. 加载 PNG 再重新保存（往返测试）", {
        let img = b::data::RGBAImage::load("examples/export_cube/export/cube_white.png")?;
        img.save("examples/export_cube/export/cube_roundtrip.png")?;
        [wait]
    });

    Ok(())
}
