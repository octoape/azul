    #![allow(dead_code, unused_imports)]
    //! Definition of azuls internal `Vec<*>` wrappers
    use crate::dll::*;
    use core::ffi::c_void;
    use core::iter;
    use core::fmt;

    use alloc::vec::{self, Vec};
    use alloc::slice;
    use alloc::string;

    use crate::gl::{
        GLint as AzGLint,
        GLuint as AzGLuint,
    };

    macro_rules! impl_vec {($struct_type:ident, $struct_name:ident) => (

        impl $struct_name {

            #[inline]
            pub fn iter(&self) -> slice::Iter<$struct_type> {
                self.as_ref().iter()
            }

            #[inline]
            pub fn into_iter(self) -> vec::IntoIter<$struct_type> {
                let v1: Vec<$struct_type> = self.into();
                v1.into_iter()
            }

            #[inline]
            pub fn iter_mut(&mut self) -> slice::IterMut<$struct_type> {
                self.as_mut().iter_mut()
            }

            #[inline]
            pub fn ptr_as_usize(&self) -> usize {
                self.ptr as usize
            }

            #[inline]
            pub fn as_mut_ptr(&mut self) -> *mut $struct_type {
                self.ptr
            }

            #[inline]
            pub fn len(&self) -> usize {
                self.len
            }

            #[inline]
            pub fn capacity(&self) -> usize {
                self.cap
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            pub fn get(&self, index: usize) -> Option<&$struct_type> {
                self.as_ref().get(index)
            }

            #[inline]
            pub unsafe fn get_unchecked(&self, index: usize) -> &$struct_type {
                self.as_ref().get_unchecked(index)
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl AsRef<[$struct_type]> for $struct_name {
            fn as_ref(&self) -> &[$struct_type] {
                unsafe { slice::from_raw_parts(self.ptr, self.len) }
            }
        }

        impl AsMut<[$struct_type]> for $struct_name {
            fn as_mut(&mut self) -> &mut [$struct_type] {
                unsafe { slice::from_raw_parts_mut (self.ptr, self.len) }
            }
        }

        impl iter::FromIterator<$struct_type> for $struct_name {
            fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = $struct_type> {
                let v: Vec<$struct_type> = Vec::from_iter(iter);
                v.into()
            }
        }

        impl From<Vec<$struct_type>> for $struct_name {
            fn from(input: Vec<$struct_type>) -> $struct_name {
                let s: &[$struct_type] = input.as_ref();
                s.into()
            }
        }

        impl From<&[$struct_type]> for $struct_name {
            fn from(input: &[$struct_type]) -> $struct_name {
                Self::copy_from(input.as_ptr(), input.len())
            }
        }

        impl From<$struct_name> for Vec<$struct_type> {
            fn from(mut input: $struct_name) -> Vec<$struct_type> {
                unsafe { slice::from_raw_parts(input.as_mut_ptr(), input.len()) }.to_vec()
            }
        }

        // Drop, Debug + Clone already implemented by default
    )}

    impl_vec!(u8,  AzU8Vec);
    impl_vec!(u32, AzU32Vec);
    impl_vec!(u32, AzScanCodeVec);
    impl_vec!(u32, AzGLuintVec);
    impl_vec!(i32, AzGLintVec);
    impl_vec!(AzStyleTransform, AzStyleTransformVec);
    impl_vec!(AzContentGroup, AzContentGroupVec);
    impl_vec!(AzCssProperty, AzCssPropertyVec);
    impl_vec!(AzSvgMultiPolygon, AzSvgMultiPolygonVec);
    impl_vec!(AzSvgPath, AzSvgPathVec);
    impl_vec!(AzVertexAttribute, AzVertexAttributeVec);
    impl_vec!(AzSvgPathElement, AzSvgPathElementVec);
    impl_vec!(AzSvgVertex, AzSvgVertexVec);
    impl_vec!(AzXWindowType, AzXWindowTypeVec);
    impl_vec!(AzVirtualKeyCode, AzVirtualKeyCodeVec);
    impl_vec!(AzCascadeInfo, AzCascadeInfoVec);
    impl_vec!(AzCssDeclaration, AzCssDeclarationVec);
    impl_vec!(AzCssPathSelector, AzCssPathSelectorVec);
    impl_vec!(AzStylesheet, AzStylesheetVec);
    impl_vec!(AzCssRuleBlock, AzCssRuleBlockVec);
    impl_vec!(AzCallbackData, AzCallbackDataVec);
    impl_vec!(AzDebugMessage, AzDebugMessageVec);
    impl_vec!(AzDom, AzDomVec);
    impl_vec!(AzString, AzStringVec);
    impl_vec!(AzStringPair, AzStringPairVec);
    impl_vec!(AzLinearColorStop, AzLinearColorStopVec);
    impl_vec!(AzRadialColorStop, AzRadialColorStopVec);
    impl_vec!(AzCascadedCssPropertyWithSource, AzCascadedCssPropertyWithSourceVec);
    impl_vec!(AzNodeId, AzNodeIdVec);
    impl_vec!(AzNode, AzNodeVec);
    impl_vec!(AzStyledNode, AzStyledNodeVec);
    impl_vec!(AzTagIdToNodeIdMapping, AzTagIdsToNodeIdsMappingVec);
    impl_vec!(AzParentWithNodeDepth, AzParentWithNodeDepthVec);
    impl_vec!(AzNodeData, AzNodeDataVec);

    impl From<vec::Vec<string::String>> for crate::vec::StringVec {
        fn from(v: vec::Vec<string::String>) -> crate::vec::StringVec {
            let mut vec: Vec<AzString> = v.into_iter().map(Into::into).collect();
            unsafe { crate::dll::az_string_vec_copy_from(vec.as_mut_ptr(), vec.len()) }
        }
    }

    impl From<crate::vec::StringVec> for vec::Vec<string::String> {
        fn from(v: crate::vec::StringVec) -> vec::Vec<string::String> {
            v
            .as_ref()
            .iter()
            .cloned()
            .map(Into::into)
            .collect()

            // delete() not necessary because StringVec is stack-allocated
        }
    }    use crate::dom::{CallbackData, Dom, IdOrClass, NodeData, NodeDataInlineCssProperty};
    use crate::css::{CssDeclaration, CssPathSelector, CssProperty, CssRuleBlock, LinearColorStop, RadialColorStop, StyleBackgroundContent, StyleBackgroundPosition, StyleBackgroundRepeat, StyleBackgroundSize, StyleTransform, Stylesheet};
    use crate::svg::{SvgMultiPolygon, SvgPath, SvgPathElement, SvgVertex};
    use crate::gl::{DebugMessage, VertexAttribute};
    use crate::window::{StringPair, VirtualKeyCode, XWindowType};
    use crate::style::{CascadeInfo, Node, ParentWithNodeDepth, StyledNode, TagIdToNodeIdMapping};
    use crate::str::String;
    use crate::callbacks::NodeId;


    /// Wrapper over a Rust-allocated `Vec<Dom>`
    #[doc(inline)] pub use crate::dll::AzDomVec as DomVec;

    impl DomVec {
        /// Creates a new, empty Rust `Vec<Dom>`
        pub fn new() -> Self { unsafe { crate::dll::az_dom_vec_new() } }
        /// Creates a new, empty Rust `Vec<Dom>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_dom_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<Dom>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzDom, len: usize) -> Self { unsafe { crate::dll::az_dom_vec_copy_from(ptr, len) } }
    }

    impl Clone for DomVec { fn clone(&self) -> Self { unsafe { crate::dll::az_dom_vec_deep_copy(self) } } }
    impl Drop for DomVec { fn drop(&mut self) { unsafe { crate::dll::az_dom_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<IdOrClass>`
    #[doc(inline)] pub use crate::dll::AzIdOrClassVec as IdOrClassVec;

    impl IdOrClassVec {
        /// Creates a new, empty Rust `Vec<IdOrClass>`
        pub fn new() -> Self { unsafe { crate::dll::az_id_or_class_vec_new() } }
        /// Creates a new, empty Rust `Vec<IdOrClass>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_id_or_class_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<IdOrClass>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzIdOrClass, len: usize) -> Self { unsafe { crate::dll::az_id_or_class_vec_copy_from(ptr, len) } }
    }

    impl Clone for IdOrClassVec { fn clone(&self) -> Self { unsafe { crate::dll::az_id_or_class_vec_deep_copy(self) } } }
    impl Drop for IdOrClassVec { fn drop(&mut self) { unsafe { crate::dll::az_id_or_class_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<NodeDataInlineCssProperty>`
    #[doc(inline)] pub use crate::dll::AzNodeDataInlineCssPropertyVec as NodeDataInlineCssPropertyVec;

    impl NodeDataInlineCssPropertyVec {
        /// Creates a new, empty Rust `Vec<NodeDataInlineCssProperty>`
        pub fn new() -> Self { unsafe { crate::dll::az_node_data_inline_css_property_vec_new() } }
        /// Creates a new, empty Rust `Vec<NodeDataInlineCssProperty>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_node_data_inline_css_property_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<NodeDataInlineCssProperty>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzNodeDataInlineCssProperty, len: usize) -> Self { unsafe { crate::dll::az_node_data_inline_css_property_vec_copy_from(ptr, len) } }
    }

    impl Clone for NodeDataInlineCssPropertyVec { fn clone(&self) -> Self { unsafe { crate::dll::az_node_data_inline_css_property_vec_deep_copy(self) } } }
    impl Drop for NodeDataInlineCssPropertyVec { fn drop(&mut self) { unsafe { crate::dll::az_node_data_inline_css_property_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundContent>`
    #[doc(inline)] pub use crate::dll::AzStyleBackgroundContentVec as StyleBackgroundContentVec;

    impl StyleBackgroundContentVec {
        /// Creates a new, empty Rust `Vec<StyleBackgroundContent>`
        pub fn new() -> Self { unsafe { crate::dll::az_style_background_content_vec_new() } }
        /// Creates a new, empty Rust `Vec<StyleBackgroundContent>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_style_background_content_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<StyleBackgroundContent>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzStyleBackgroundContent, len: usize) -> Self { unsafe { crate::dll::az_style_background_content_vec_copy_from(ptr, len) } }
    }

    impl Clone for StyleBackgroundContentVec { fn clone(&self) -> Self { unsafe { crate::dll::az_style_background_content_vec_deep_copy(self) } } }
    impl Drop for StyleBackgroundContentVec { fn drop(&mut self) { unsafe { crate::dll::az_style_background_content_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundPosition>`
    #[doc(inline)] pub use crate::dll::AzStyleBackgroundPositionVec as StyleBackgroundPositionVec;

    impl StyleBackgroundPositionVec {
        /// Creates a new, empty Rust `Vec<StyleBackgroundPosition>`
        pub fn new() -> Self { unsafe { crate::dll::az_style_background_position_vec_new() } }
        /// Creates a new, empty Rust `Vec<StyleBackgroundPosition>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_style_background_position_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<StyleBackgroundPosition>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzStyleBackgroundPosition, len: usize) -> Self { unsafe { crate::dll::az_style_background_position_vec_copy_from(ptr, len) } }
    }

    impl Clone for StyleBackgroundPositionVec { fn clone(&self) -> Self { unsafe { crate::dll::az_style_background_position_vec_deep_copy(self) } } }
    impl Drop for StyleBackgroundPositionVec { fn drop(&mut self) { unsafe { crate::dll::az_style_background_position_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundRepeat>`
    #[doc(inline)] pub use crate::dll::AzStyleBackgroundRepeatVec as StyleBackgroundRepeatVec;

    impl StyleBackgroundRepeatVec {
        /// Creates a new, empty Rust `Vec<StyleBackgroundRepeat>`
        pub fn new() -> Self { unsafe { crate::dll::az_style_background_repeat_vec_new() } }
        /// Creates a new, empty Rust `Vec<StyleBackgroundRepeat>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_style_background_repeat_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<StyleBackgroundRepeat>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzStyleBackgroundRepeat, len: usize) -> Self { unsafe { crate::dll::az_style_background_repeat_vec_copy_from(ptr, len) } }
    }

    impl Clone for StyleBackgroundRepeatVec { fn clone(&self) -> Self { unsafe { crate::dll::az_style_background_repeat_vec_deep_copy(self) } } }
    impl Drop for StyleBackgroundRepeatVec { fn drop(&mut self) { unsafe { crate::dll::az_style_background_repeat_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundSize>`
    #[doc(inline)] pub use crate::dll::AzStyleBackgroundSizeVec as StyleBackgroundSizeVec;

    impl StyleBackgroundSizeVec {
        /// Creates a new, empty Rust `Vec<StyleBackgroundSize>`
        pub fn new() -> Self { unsafe { crate::dll::az_style_background_size_vec_new() } }
        /// Creates a new, empty Rust `Vec<StyleBackgroundSize>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_style_background_size_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<StyleBackgroundSize>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzStyleBackgroundSize, len: usize) -> Self { unsafe { crate::dll::az_style_background_size_vec_copy_from(ptr, len) } }
    }

    impl Clone for StyleBackgroundSizeVec { fn clone(&self) -> Self { unsafe { crate::dll::az_style_background_size_vec_deep_copy(self) } } }
    impl Drop for StyleBackgroundSizeVec { fn drop(&mut self) { unsafe { crate::dll::az_style_background_size_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<StyleTransform>`
    #[doc(inline)] pub use crate::dll::AzStyleTransformVec as StyleTransformVec;

    impl StyleTransformVec {
        /// Creates a new, empty Rust `Vec<StyleTransform>`
        pub fn new() -> Self { unsafe { crate::dll::az_style_transform_vec_new() } }
        /// Creates a new, empty Rust `Vec<StyleTransform>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_style_transform_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<StyleTransform>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzStyleTransform, len: usize) -> Self { unsafe { crate::dll::az_style_transform_vec_copy_from(ptr, len) } }
    }

    impl Clone for StyleTransformVec { fn clone(&self) -> Self { unsafe { crate::dll::az_style_transform_vec_deep_copy(self) } } }
    impl Drop for StyleTransformVec { fn drop(&mut self) { unsafe { crate::dll::az_style_transform_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<CssProperty>`
    #[doc(inline)] pub use crate::dll::AzCssPropertyVec as CssPropertyVec;

    impl CssPropertyVec {
        /// Creates a new, empty Rust `Vec<CssProperty>`
        pub fn new() -> Self { unsafe { crate::dll::az_css_property_vec_new() } }
        /// Creates a new, empty Rust `Vec<CssProperty>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_css_property_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<CssProperty>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzCssProperty, len: usize) -> Self { unsafe { crate::dll::az_css_property_vec_copy_from(ptr, len) } }
    }

    impl Clone for CssPropertyVec { fn clone(&self) -> Self { unsafe { crate::dll::az_css_property_vec_deep_copy(self) } } }
    impl Drop for CssPropertyVec { fn drop(&mut self) { unsafe { crate::dll::az_css_property_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<SvgMultiPolygon>`
    #[doc(inline)] pub use crate::dll::AzSvgMultiPolygonVec as SvgMultiPolygonVec;

    impl SvgMultiPolygonVec {
        /// Creates a new, empty Rust `Vec<SvgMultiPolygon>`
        pub fn new() -> Self { unsafe { crate::dll::az_svg_multi_polygon_vec_new() } }
        /// Creates a new, empty Rust `Vec<SvgMultiPolygon>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_svg_multi_polygon_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<SvgMultiPolygon>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzSvgMultiPolygon, len: usize) -> Self { unsafe { crate::dll::az_svg_multi_polygon_vec_copy_from(ptr, len) } }
    }

    impl Clone for SvgMultiPolygonVec { fn clone(&self) -> Self { unsafe { crate::dll::az_svg_multi_polygon_vec_deep_copy(self) } } }
    impl Drop for SvgMultiPolygonVec { fn drop(&mut self) { unsafe { crate::dll::az_svg_multi_polygon_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<SvgPath>`
    #[doc(inline)] pub use crate::dll::AzSvgPathVec as SvgPathVec;

    impl SvgPathVec {
        /// Creates a new, empty Rust `Vec<SvgPath>`
        pub fn new() -> Self { unsafe { crate::dll::az_svg_path_vec_new() } }
        /// Creates a new, empty Rust `Vec<SvgPath>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_svg_path_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<SvgPath>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzSvgPath, len: usize) -> Self { unsafe { crate::dll::az_svg_path_vec_copy_from(ptr, len) } }
    }

    impl Clone for SvgPathVec { fn clone(&self) -> Self { unsafe { crate::dll::az_svg_path_vec_deep_copy(self) } } }
    impl Drop for SvgPathVec { fn drop(&mut self) { unsafe { crate::dll::az_svg_path_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<VertexAttribute>`
    #[doc(inline)] pub use crate::dll::AzVertexAttributeVec as VertexAttributeVec;

    impl VertexAttributeVec {
        /// Creates a new, empty Rust `Vec<VertexAttribute>`
        pub fn new() -> Self { unsafe { crate::dll::az_vertex_attribute_vec_new() } }
        /// Creates a new, empty Rust `Vec<VertexAttribute>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_vertex_attribute_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<VertexAttribute>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzVertexAttribute, len: usize) -> Self { unsafe { crate::dll::az_vertex_attribute_vec_copy_from(ptr, len) } }
    }

    impl Clone for VertexAttributeVec { fn clone(&self) -> Self { unsafe { crate::dll::az_vertex_attribute_vec_deep_copy(self) } } }
    impl Drop for VertexAttributeVec { fn drop(&mut self) { unsafe { crate::dll::az_vertex_attribute_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `VertexAttribute`
    #[doc(inline)] pub use crate::dll::AzSvgPathElementVec as SvgPathElementVec;

    impl SvgPathElementVec {
        /// Creates a new, empty Rust `Vec<SvgPathElement>`
        pub fn new() -> Self { unsafe { crate::dll::az_svg_path_element_vec_new() } }
        /// Creates a new, empty Rust `Vec<SvgPathElement>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_svg_path_element_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<SvgPathElement>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzSvgPathElement, len: usize) -> Self { unsafe { crate::dll::az_svg_path_element_vec_copy_from(ptr, len) } }
    }

    impl Clone for SvgPathElementVec { fn clone(&self) -> Self { unsafe { crate::dll::az_svg_path_element_vec_deep_copy(self) } } }
    impl Drop for SvgPathElementVec { fn drop(&mut self) { unsafe { crate::dll::az_svg_path_element_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `SvgVertex`
    #[doc(inline)] pub use crate::dll::AzSvgVertexVec as SvgVertexVec;

    impl SvgVertexVec {
        /// Creates a new, empty Rust `Vec<SvgVertex>`
        pub fn new() -> Self { unsafe { crate::dll::az_svg_vertex_vec_new() } }
        /// Creates a new, empty Rust `Vec<SvgVertex>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_svg_vertex_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<SvgVertex>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzSvgVertex, len: usize) -> Self { unsafe { crate::dll::az_svg_vertex_vec_copy_from(ptr, len) } }
    }

    impl Clone for SvgVertexVec { fn clone(&self) -> Self { unsafe { crate::dll::az_svg_vertex_vec_deep_copy(self) } } }
    impl Drop for SvgVertexVec { fn drop(&mut self) { unsafe { crate::dll::az_svg_vertex_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<u32>`
    #[doc(inline)] pub use crate::dll::AzU32Vec as U32Vec;

    impl U32Vec {
        /// Creates a new, empty Rust `Vec<u32>`
        pub fn new() -> Self { unsafe { crate::dll::az_u32_vec_new() } }
        /// Creates a new, empty Rust `Vec<u32>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_u32_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<u32>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const u32, len: usize) -> Self { unsafe { crate::dll::az_u32_vec_copy_from(ptr, len) } }
    }

    impl Clone for U32Vec { fn clone(&self) -> Self { unsafe { crate::dll::az_u32_vec_deep_copy(self) } } }
    impl Drop for U32Vec { fn drop(&mut self) { unsafe { crate::dll::az_u32_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `XWindowType`
    #[doc(inline)] pub use crate::dll::AzXWindowTypeVec as XWindowTypeVec;

    impl XWindowTypeVec {
        /// Creates a new, empty Rust `Vec<XWindowType>`
        pub fn new() -> Self { unsafe { crate::dll::az_x_window_type_vec_new() } }
        /// Creates a new, empty Rust `Vec<XWindowType>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_x_window_type_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<XWindowType>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzXWindowType, len: usize) -> Self { unsafe { crate::dll::az_x_window_type_vec_copy_from(ptr, len) } }
    }

    impl Clone for XWindowTypeVec { fn clone(&self) -> Self { unsafe { crate::dll::az_x_window_type_vec_deep_copy(self) } } }
    impl Drop for XWindowTypeVec { fn drop(&mut self) { unsafe { crate::dll::az_x_window_type_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `VirtualKeyCode`
    #[doc(inline)] pub use crate::dll::AzVirtualKeyCodeVec as VirtualKeyCodeVec;

    impl VirtualKeyCodeVec {
        /// Creates a new, empty Rust `Vec<VirtualKeyCode>`
        pub fn new() -> Self { unsafe { crate::dll::az_virtual_key_code_vec_new() } }
        /// Creates a new, empty Rust `Vec<VirtualKeyCode>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_virtual_key_code_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<VirtualKeyCode>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzVirtualKeyCode, len: usize) -> Self { unsafe { crate::dll::az_virtual_key_code_vec_copy_from(ptr, len) } }
    }

    impl Clone for VirtualKeyCodeVec { fn clone(&self) -> Self { unsafe { crate::dll::az_virtual_key_code_vec_deep_copy(self) } } }
    impl Drop for VirtualKeyCodeVec { fn drop(&mut self) { unsafe { crate::dll::az_virtual_key_code_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `CascadeInfo`
    #[doc(inline)] pub use crate::dll::AzCascadeInfoVec as CascadeInfoVec;

    impl CascadeInfoVec {
        /// Creates a new, empty Rust `Vec<CascadeInfo>`
        pub fn new() -> Self { unsafe { crate::dll::az_cascade_info_vec_new() } }
        /// Creates a new, empty Rust `Vec<CascadeInfo>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_cascade_info_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<CascadeInfo>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzCascadeInfo, len: usize) -> Self { unsafe { crate::dll::az_cascade_info_vec_copy_from(ptr, len) } }
    }

    impl Clone for CascadeInfoVec { fn clone(&self) -> Self { unsafe { crate::dll::az_cascade_info_vec_deep_copy(self) } } }
    impl Drop for CascadeInfoVec { fn drop(&mut self) { unsafe { crate::dll::az_cascade_info_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `ScanCode`
    #[doc(inline)] pub use crate::dll::AzScanCodeVec as ScanCodeVec;

    impl ScanCodeVec {
        /// Creates a new, empty Rust `Vec<ScanCode>`
        pub fn new() -> Self { unsafe { crate::dll::az_scan_code_vec_new() } }
        /// Creates a new, empty Rust `Vec<ScanCode>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_scan_code_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<ScanCode>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const u32, len: usize) -> Self { unsafe { crate::dll::az_scan_code_vec_copy_from(ptr, len) } }
    }

    impl Clone for ScanCodeVec { fn clone(&self) -> Self { unsafe { crate::dll::az_scan_code_vec_deep_copy(self) } } }
    impl Drop for ScanCodeVec { fn drop(&mut self) { unsafe { crate::dll::az_scan_code_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `CssDeclaration`
    #[doc(inline)] pub use crate::dll::AzCssDeclarationVec as CssDeclarationVec;

    impl CssDeclarationVec {
        /// Creates a new, empty Rust `Vec<CssDeclaration>`
        pub fn new() -> Self { unsafe { crate::dll::az_css_declaration_vec_new() } }
        /// Creates a new, empty Rust `Vec<CssDeclaration>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_css_declaration_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<CssDeclaration>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzCssDeclaration, len: usize) -> Self { unsafe { crate::dll::az_css_declaration_vec_copy_from(ptr, len) } }
    }

    impl Clone for CssDeclarationVec { fn clone(&self) -> Self { unsafe { crate::dll::az_css_declaration_vec_deep_copy(self) } } }
    impl Drop for CssDeclarationVec { fn drop(&mut self) { unsafe { crate::dll::az_css_declaration_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `CssPathSelector`
    #[doc(inline)] pub use crate::dll::AzCssPathSelectorVec as CssPathSelectorVec;

    impl CssPathSelectorVec {
        /// Creates a new, empty Rust `Vec<CssPathSelector>`
        pub fn new() -> Self { unsafe { crate::dll::az_css_path_selector_vec_new() } }
        /// Creates a new, empty Rust `Vec<CssPathSelector>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_css_path_selector_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<CssPathSelector>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzCssPathSelector, len: usize) -> Self { unsafe { crate::dll::az_css_path_selector_vec_copy_from(ptr, len) } }
    }

    impl Clone for CssPathSelectorVec { fn clone(&self) -> Self { unsafe { crate::dll::az_css_path_selector_vec_deep_copy(self) } } }
    impl Drop for CssPathSelectorVec { fn drop(&mut self) { unsafe { crate::dll::az_css_path_selector_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Stylesheet`
    #[doc(inline)] pub use crate::dll::AzStylesheetVec as StylesheetVec;

    impl StylesheetVec {
        /// Creates a new, empty Rust `Vec<Stylesheet>`
        pub fn new() -> Self { unsafe { crate::dll::az_stylesheet_vec_new() } }
        /// Creates a new, empty Rust `Vec<Stylesheet>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_stylesheet_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<Stylesheet>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzStylesheet, len: usize) -> Self { unsafe { crate::dll::az_stylesheet_vec_copy_from(ptr, len) } }
    }

    impl Clone for StylesheetVec { fn clone(&self) -> Self { unsafe { crate::dll::az_stylesheet_vec_deep_copy(self) } } }
    impl Drop for StylesheetVec { fn drop(&mut self) { unsafe { crate::dll::az_stylesheet_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `CssRuleBlock`
    #[doc(inline)] pub use crate::dll::AzCssRuleBlockVec as CssRuleBlockVec;

    impl CssRuleBlockVec {
        /// Creates a new, empty Rust `Vec<CssRuleBlock>`
        pub fn new() -> Self { unsafe { crate::dll::az_css_rule_block_vec_new() } }
        /// Creates a new, empty Rust `Vec<CssRuleBlock>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_css_rule_block_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<CssRuleBlock>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzCssRuleBlock, len: usize) -> Self { unsafe { crate::dll::az_css_rule_block_vec_copy_from(ptr, len) } }
    }

    impl Clone for CssRuleBlockVec { fn clone(&self) -> Self { unsafe { crate::dll::az_css_rule_block_vec_deep_copy(self) } } }
    impl Drop for CssRuleBlockVec { fn drop(&mut self) { unsafe { crate::dll::az_css_rule_block_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `U8Vec`
    #[doc(inline)] pub use crate::dll::AzU8Vec as U8Vec;

    impl U8Vec {
        /// Creates a new, empty Rust `Vec<u8>`
        pub fn new() -> Self { unsafe { crate::dll::az_u8_vec_new() } }
        /// Creates a new, empty Rust `Vec<u8>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_u8_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<u8>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const u8, len: usize) -> Self { unsafe { crate::dll::az_u8_vec_copy_from(ptr, len) } }
    }

    impl Clone for U8Vec { fn clone(&self) -> Self { unsafe { crate::dll::az_u8_vec_deep_copy(self) } } }
    impl Drop for U8Vec { fn drop(&mut self) { unsafe { crate::dll::az_u8_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `CallbackData`
    #[doc(inline)] pub use crate::dll::AzCallbackDataVec as CallbackDataVec;

    impl CallbackDataVec {
        /// Creates a new, empty Rust `Vec<CallbackData>`
        pub fn new() -> Self { unsafe { crate::dll::az_callback_data_vec_new() } }
        /// Creates a new, empty Rust `Vec<CallbackData>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_callback_data_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<CallbackData>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzCallbackData, len: usize) -> Self { unsafe { crate::dll::az_callback_data_vec_copy_from(ptr, len) } }
    }

    impl Clone for CallbackDataVec { fn clone(&self) -> Self { unsafe { crate::dll::az_callback_data_vec_deep_copy(self) } } }
    impl Drop for CallbackDataVec { fn drop(&mut self) { unsafe { crate::dll::az_callback_data_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `Vec<DebugMessage>`
    #[doc(inline)] pub use crate::dll::AzDebugMessageVec as DebugMessageVec;

    impl DebugMessageVec {
        /// Creates a new, empty Rust `Vec<DebugMessage>`
        pub fn new() -> Self { unsafe { crate::dll::az_debug_message_vec_new() } }
        /// Creates a new, empty Rust `Vec<DebugMessage>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_debug_message_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<DebugMessage>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzDebugMessage, len: usize) -> Self { unsafe { crate::dll::az_debug_message_vec_copy_from(ptr, len) } }
    }

    impl Clone for DebugMessageVec { fn clone(&self) -> Self { unsafe { crate::dll::az_debug_message_vec_deep_copy(self) } } }
    impl Drop for DebugMessageVec { fn drop(&mut self) { unsafe { crate::dll::az_debug_message_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `U32Vec`
    #[doc(inline)] pub use crate::dll::AzGLuintVec as GLuintVec;

    impl GLuintVec {
        /// Creates a new, empty Rust `Vec<u32>`
        pub fn new() -> Self { unsafe { crate::dll::az_g_luint_vec_new() } }
        /// Creates a new, empty Rust `Vec<u32>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_g_luint_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<u32>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const u32, len: usize) -> Self { unsafe { crate::dll::az_g_luint_vec_copy_from(ptr, len) } }
    }

    impl Clone for GLuintVec { fn clone(&self) -> Self { unsafe { crate::dll::az_g_luint_vec_deep_copy(self) } } }
    impl Drop for GLuintVec { fn drop(&mut self) { unsafe { crate::dll::az_g_luint_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `GLintVec`
    #[doc(inline)] pub use crate::dll::AzGLintVec as GLintVec;

    impl GLintVec {
        /// Creates a new, empty Rust `Vec<GLint>`
        pub fn new() -> Self { unsafe { crate::dll::az_g_lint_vec_new() } }
        /// Creates a new, empty Rust `Vec<GLint>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_g_lint_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<GLint>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const i32, len: usize) -> Self { unsafe { crate::dll::az_g_lint_vec_copy_from(ptr, len) } }
    }

    impl Clone for GLintVec { fn clone(&self) -> Self { unsafe { crate::dll::az_g_lint_vec_deep_copy(self) } } }
    impl Drop for GLintVec { fn drop(&mut self) { unsafe { crate::dll::az_g_lint_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `StringVec`
    #[doc(inline)] pub use crate::dll::AzStringVec as StringVec;

    impl StringVec {
        /// Creates a new, empty Rust `Vec<String>`
        pub fn new() -> Self { unsafe { crate::dll::az_string_vec_new() } }
        /// Creates a new, empty Rust `Vec<String>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_string_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<String>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzString, len: usize) -> Self { unsafe { crate::dll::az_string_vec_copy_from(ptr, len) } }
    }

    impl Clone for StringVec { fn clone(&self) -> Self { unsafe { crate::dll::az_string_vec_deep_copy(self) } } }
    impl Drop for StringVec { fn drop(&mut self) { unsafe { crate::dll::az_string_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `StringPairVec`
    #[doc(inline)] pub use crate::dll::AzStringPairVec as StringPairVec;

    impl StringPairVec {
        /// Creates a new, empty Rust `Vec<StringPair>`
        pub fn new() -> Self { unsafe { crate::dll::az_string_pair_vec_new() } }
        /// Creates a new, empty Rust `Vec<StringPair>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_string_pair_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<StringPair>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzStringPair, len: usize) -> Self { unsafe { crate::dll::az_string_pair_vec_copy_from(ptr, len) } }
    }

    impl Clone for StringPairVec { fn clone(&self) -> Self { unsafe { crate::dll::az_string_pair_vec_deep_copy(self) } } }
    impl Drop for StringPairVec { fn drop(&mut self) { unsafe { crate::dll::az_string_pair_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `LinearColorStopVec`
    #[doc(inline)] pub use crate::dll::AzLinearColorStopVec as LinearColorStopVec;

    impl LinearColorStopVec {
        /// Creates a new, empty Rust `Vec<LinearColorStop>`
        pub fn new() -> Self { unsafe { crate::dll::az_linear_color_stop_vec_new() } }
        /// Creates a new, empty Rust `Vec<LinearColorStop>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_linear_color_stop_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<LinearColorStop>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzLinearColorStop, len: usize) -> Self { unsafe { crate::dll::az_linear_color_stop_vec_copy_from(ptr, len) } }
    }

    impl Clone for LinearColorStopVec { fn clone(&self) -> Self { unsafe { crate::dll::az_linear_color_stop_vec_deep_copy(self) } } }
    impl Drop for LinearColorStopVec { fn drop(&mut self) { unsafe { crate::dll::az_linear_color_stop_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `RadialColorStopVec`
    #[doc(inline)] pub use crate::dll::AzRadialColorStopVec as RadialColorStopVec;

    impl RadialColorStopVec {
        /// Creates a new, empty Rust `Vec<RadialColorStop>`
        pub fn new() -> Self { unsafe { crate::dll::az_radial_color_stop_vec_new() } }
        /// Creates a new, empty Rust `Vec<RadialColorStop>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_radial_color_stop_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<RadialColorStop>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzRadialColorStop, len: usize) -> Self { unsafe { crate::dll::az_radial_color_stop_vec_copy_from(ptr, len) } }
    }

    impl Clone for RadialColorStopVec { fn clone(&self) -> Self { unsafe { crate::dll::az_radial_color_stop_vec_deep_copy(self) } } }
    impl Drop for RadialColorStopVec { fn drop(&mut self) { unsafe { crate::dll::az_radial_color_stop_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `NodeIdVec`
    #[doc(inline)] pub use crate::dll::AzNodeIdVec as NodeIdVec;

    impl NodeIdVec {
        /// Creates a new, empty Rust `Vec<NodeId>`
        pub fn new() -> Self { unsafe { crate::dll::az_node_id_vec_new() } }
        /// Creates a new, empty Rust `Vec<NodeId>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_node_id_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<NodeId>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzNodeId, len: usize) -> Self { unsafe { crate::dll::az_node_id_vec_copy_from(ptr, len) } }
    }

    impl Clone for NodeIdVec { fn clone(&self) -> Self { unsafe { crate::dll::az_node_id_vec_deep_copy(self) } } }
    impl Drop for NodeIdVec { fn drop(&mut self) { unsafe { crate::dll::az_node_id_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `NodeVec`
    #[doc(inline)] pub use crate::dll::AzNodeVec as NodeVec;

    impl NodeVec {
        /// Creates a new, empty Rust `Vec<Node>`
        pub fn new() -> Self { unsafe { crate::dll::az_node_vec_new() } }
        /// Creates a new, empty Rust `Vec<Node>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_node_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<Node>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzNode, len: usize) -> Self { unsafe { crate::dll::az_node_vec_copy_from(ptr, len) } }
    }

    impl Clone for NodeVec { fn clone(&self) -> Self { unsafe { crate::dll::az_node_vec_deep_copy(self) } } }
    impl Drop for NodeVec { fn drop(&mut self) { unsafe { crate::dll::az_node_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `StyledNodeVec`
    #[doc(inline)] pub use crate::dll::AzStyledNodeVec as StyledNodeVec;

    impl StyledNodeVec {
        /// Creates a new, empty Rust `Vec<StyledNode>`
        pub fn new() -> Self { unsafe { crate::dll::az_styled_node_vec_new() } }
        /// Creates a new, empty Rust `Vec<StyledNode>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_styled_node_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<StyledNode>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzStyledNode, len: usize) -> Self { unsafe { crate::dll::az_styled_node_vec_copy_from(ptr, len) } }
    }

    impl Clone for StyledNodeVec { fn clone(&self) -> Self { unsafe { crate::dll::az_styled_node_vec_deep_copy(self) } } }
    impl Drop for StyledNodeVec { fn drop(&mut self) { unsafe { crate::dll::az_styled_node_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `TagIdsToNodeIdsMappingVec`
    #[doc(inline)] pub use crate::dll::AzTagIdsToNodeIdsMappingVec as TagIdsToNodeIdsMappingVec;

    impl TagIdsToNodeIdsMappingVec {
        /// Creates a new, empty Rust `Vec<TagIdToNodeIdMapping>`
        pub fn new() -> Self { unsafe { crate::dll::az_tag_ids_to_node_ids_mapping_vec_new() } }
        /// Creates a new, empty Rust `Vec<TagIdToNodeIdMapping>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_tag_ids_to_node_ids_mapping_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<TagIdToNodeIdMapping>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzTagIdToNodeIdMapping, len: usize) -> Self { unsafe { crate::dll::az_tag_ids_to_node_ids_mapping_vec_copy_from(ptr, len) } }
    }

    impl Clone for TagIdsToNodeIdsMappingVec { fn clone(&self) -> Self { unsafe { crate::dll::az_tag_ids_to_node_ids_mapping_vec_deep_copy(self) } } }
    impl Drop for TagIdsToNodeIdsMappingVec { fn drop(&mut self) { unsafe { crate::dll::az_tag_ids_to_node_ids_mapping_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `ParentWithNodeDepthVec`
    #[doc(inline)] pub use crate::dll::AzParentWithNodeDepthVec as ParentWithNodeDepthVec;

    impl ParentWithNodeDepthVec {
        /// Creates a new, empty Rust `Vec<ParentWithNodeDepth>`
        pub fn new() -> Self { unsafe { crate::dll::az_parent_with_node_depth_vec_new() } }
        /// Creates a new, empty Rust `Vec<ParentWithNodeDepth>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_parent_with_node_depth_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<ParentWithNodeDepth>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzParentWithNodeDepth, len: usize) -> Self { unsafe { crate::dll::az_parent_with_node_depth_vec_copy_from(ptr, len) } }
    }

    impl Clone for ParentWithNodeDepthVec { fn clone(&self) -> Self { unsafe { crate::dll::az_parent_with_node_depth_vec_deep_copy(self) } } }
    impl Drop for ParentWithNodeDepthVec { fn drop(&mut self) { unsafe { crate::dll::az_parent_with_node_depth_vec_delete(self) }; } }


    /// Wrapper over a Rust-allocated `NodeDataVec`
    #[doc(inline)] pub use crate::dll::AzNodeDataVec as NodeDataVec;

    impl NodeDataVec {
        /// Creates a new, empty Rust `Vec<NodeData>`
        pub fn new() -> Self { unsafe { crate::dll::az_node_data_vec_new() } }
        /// Creates a new, empty Rust `Vec<NodeData>` with a given, pre-allocated capacity
        pub fn with_capacity(cap: usize) -> Self { unsafe { crate::dll::az_node_data_vec_with_capacity(cap) } }
        /// Creates + allocates a Rust `Vec<NodeData>` by **copying** it from a bytes source
        pub fn copy_from(ptr: *const AzNodeData, len: usize) -> Self { unsafe { crate::dll::az_node_data_vec_copy_from(ptr, len) } }
    }

    impl Clone for NodeDataVec { fn clone(&self) -> Self { unsafe { crate::dll::az_node_data_vec_deep_copy(self) } } }
    impl Drop for NodeDataVec { fn drop(&mut self) { unsafe { crate::dll::az_node_data_vec_delete(self) }; } }
