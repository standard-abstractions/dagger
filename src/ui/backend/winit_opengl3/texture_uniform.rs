use glium::{
	program::BlockLayout,
	uniforms::{LayoutMismatchError, UniformType},
	implement_buffer_content,
};

// !! Do not edit this without updating the UniformBlock implementation !!
pub struct TextureBuffer<'a> {
	pub textures: [glium::texture::TextureHandle<'a>],
}
implement_buffer_content!(TextureBuffer<'a>);

impl<'a> glium::uniforms::UniformBlock for TextureBuffer<'a> {
    fn build_layout(base_offset: usize) -> BlockLayout {
        BlockLayout::Struct {
            members: vec![
                (
                    String::from("textures"),
                    BlockLayout::DynamicSizedArray {
                        content: Box::new(BlockLayout::BasicType {
                            ty: UniformType::Image2d,
                            offset_in_buffer: base_offset,
                        })
                    }
                )
            ]
        }
    }

	fn matches(_: &glium::program::BlockLayout, _: usize) -> Result<(), LayoutMismatchError> {
		Ok(()) // The BlockLayout is hand-crafted, so there should be no need to check.
	}
}
