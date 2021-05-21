/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    bezier.rs
@brief   Part of geometry library
 */

//a Notes
//
//

//a Imports
use std::cell::RefCell;

//a ByteBuffer
//tp ByteBuffer
/// A trait for all types that are to be used as sources of data for
/// buffers of, e.g. vertex data, indices, etc
///
/// The data is viewed by OpenGL as a pointer and byte length; these
/// methods provide access to the data in that way.
///
/// These methods are all safe - any use of the information they
/// provide may be unsafe.
pub trait ByteBuffer {
    /// Get the length of the data buffer in bytes
    fn byte_length(&self) -> usize;
    /// Borrow the data as an array of bytes
    fn borrow_bytes<'a> (&'a self) -> &'a [u8];
    /// Return a pointer to the first byte of the data contents
    fn as_ptr(&self) -> *const u8 { self.borrow_bytes().as_ptr() }
}

//ti ByteBuffer for [T; N]
/// Implement ByteBuffer for [T]
impl <T, const N:usize> ByteBuffer for [T; N] {
    //fp byte_length
    fn byte_length(&self) -> usize {
        std::mem::size_of::<T>() * N
    }

    //fp borrow_bytes
    fn borrow_bytes<'a>(&'a self) -> &'a [u8] {
	    unsafe { std::mem::transmute::<&[T], &[u8]>(self) }
    }

    //zz All done
}

//ti ByteBuffer for Vec
/// Implement ByteBuffer for Vec
impl <T> ByteBuffer for Vec<T> {
    //fp byte_length
    fn byte_length(&self) -> usize {
        std::mem::size_of::<T>() * self.len()
    }

    //fp borrow_bytes
    fn borrow_bytes<'a>(&'a self) -> &'a [u8] {
	    unsafe { std::mem::transmute::<&[T], &[u8]>(&self) }
    }

    //zz All done
}

//a Indices
//tp Indices
/// A buffer for use with OpenGL index data
///
/// This does not support a 'View' concept; the indices for a
/// primitive should be contiguous within an `Indices` data buffer,
/// and all of the same type. Selecting the indices to use for a
/// particular draw call is part of the draw function itself.
pub struct Indices<'a> {
    /// Data buffer itself
    data        : &'a [u8],
    /// Offset in to the data buffer for the first byte
    byte_offset : usize,
    /// Length of data used in the buffer
    byte_length : usize,
    /// if a gl buffer then bound to data[byte_offset] .. + byte_length
    rc_gl_buffer   : RefCell<gl::types::GLuint>,
}

//ip Indices
impl <'a> Indices<'a> {

    //fp new
    /// Create a new `Data` given a buffer, offset and length; if the
    /// length is zero then the whole of the data buffer post offset
    /// is used
    ///
    /// If offset and length are both zero, then all the data is used
    ///
    /// This function can be invoked prior to the OpenGL context being
    /// created; this performs no OpenGL calls
    pub fn new<B:ByteBuffer>(data:&'a B, byte_offset:usize, byte_length:usize) -> Self {
        let byte_length = {
            if byte_length == 0 { data.byte_length()-byte_offset } else { byte_length }
        };
        let rc_gl_buffer = RefCell::new(0);
        let data = data.borrow_bytes();
        Self { data, byte_offset, byte_length, rc_gl_buffer }
    }

    //ap gl_buffer
    /// Get the gl_buffer associated with the data, assuming its
    /// `gl_create` method has been invoked at least once
    pub fn gl_buffer(&self) -> gl::types::GLuint {
        *self.rc_gl_buffer.borrow()
    }

    //mp gl_create
    /// Create the OpenGL ELEMENT_ARRAY_BUFFER using STATIC_DRAW - this copies the data in to OpenGL
    ///
    /// If this method is invoked more than once, only one OpenGL buffer is created
    pub fn gl_create(&self) {
        let gl_buffer = *self.rc_gl_buffer.borrow();
        if gl_buffer == 0 {
            unsafe {
                gl::GenBuffers(1, self.rc_gl_buffer.as_ptr() );
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *self.rc_gl_buffer.borrow() );
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                               self.byte_length as gl::types::GLsizeiptr,
                               self.data.as_ptr() as *const gl::types::GLvoid,
                               gl::STATIC_DRAW );
            }
        }
    }

    //mp gl_bind
    /// Bind the data to the VAO
    pub fn gl_bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,
                           self.gl_buffer() );
        }
    }

    // def hier_debug(self, hier:Hierarchy) -> Hierarchy:
    //     hier.add(f"Indices {self.byte_offset} {self.byte_length}")
    //   return hier
}

//ip Drop for Indices
impl <'a> Drop for Indices<'a> {
    //fp drop
    /// If an OpenGL buffer has been created for this then delete it
    fn drop(&mut self) {
        if self.gl_buffer() != 0 {
            unsafe {
                gl::DeleteBuffers(1, self.rc_gl_buffer.as_ptr() );
            }
        }
    }
}

//a Data
//tp Data
/// A data buffer for use with OpenGL vertex data
///
/// A data buffer may contain a lot of data per vertex, such as
/// position, normal, tangent, color etc.  a `View` on the data is
/// then a subset of this data - perhaps picking out just the
/// position, for example, for a set of vertices
///
/// The data buffer may, indeed, contain data for more than one object
/// - and the objects may have different data per vertex. The data
/// buffer is pretty free-form, it is a `View` on the `Data` which
/// identifies the object it applies to, and the vertex attributes
/// required
///
/// A data buffer may then be used by many `View`s. Each `View` may be
/// used by many primitives for a single model; alternatively,
/// primitives may have their own individual Views.
///
/// Of course the model may be instantiated many times in a single scene.
///
/// OpenGL will have one copy of the data for all the primitives and models.
pub struct Data<'a> {
    /// Data buffer itself
    data        : &'a [u8],
    /// Offset in to the data buffer for the first byte
    byte_offset : usize,
    /// Length of data used in the buffer
    byte_length : usize,
    /// if a gl buffer then bound to data[byte_offset] .. + byte_length
    rc_gl_buffer   : RefCell<gl::types::GLuint>,
}

//ip Data
impl <'a> Data<'a> {
    //fp new
    /// Create a new `Data` given a buffer, offset and length; if the
    /// length is zero then the whole of the data buffer post offset
    /// is used
    ///
    /// If offset and length are both zero, then all the data is used
    ///
    /// This function can be invoked prior to the OpenGL context being
    /// created; this performs no OpenGL calls
    pub fn new<B:ByteBuffer>(data:&'a B, byte_offset:usize, byte_length:usize) -> Self {
        let byte_length = {
            if byte_length == 0 { data.byte_length()-byte_offset } else { byte_length }
        };
        let rc_gl_buffer = RefCell::new(0);
        let data = data.borrow_bytes();
        Self { data, byte_offset, byte_length, rc_gl_buffer }
    }

    //ap gl_buffer
    /// Get the gl_buffer associated with the data, assuming its
    /// `gl_create` method has been invoked at least once
    pub fn gl_buffer(&self) -> gl::types::GLuint {
        *self.rc_gl_buffer.borrow()
    }

    //mp gl_create
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    ///
    /// If this method is invoked more than once, only one OpenGL buffer is created
    pub fn gl_create(&self) {
        let gl_buffer = *self.rc_gl_buffer.borrow();
        if gl_buffer == 0 {
            unsafe {
                gl::GenBuffers(1, self.rc_gl_buffer.as_ptr() );
                gl::BindBuffer(gl::ARRAY_BUFFER, *self.rc_gl_buffer.borrow());
                gl::BufferData(gl::ARRAY_BUFFER,
                               self.byte_length as gl::types::GLsizeiptr,
                               self.data.as_ptr() as *const gl::types::GLvoid,
                               gl::STATIC_DRAW );
            }
        }
    }

    //zz All done
}

//ip Drop for Data
impl <'a> Drop for Data<'a> {
    //fp drop
    /// If an OpenGL buffer has been created for this then delete it
    fn drop(&mut self) {
        if self.gl_buffer() != 0 {
            unsafe {
                gl::DeleteBuffers(1, self.rc_gl_buffer.as_ptr() );
            }
        }
    }
}

//a View
/// A subset of a `Data`, used for vertex attributes;
/// hence for use in a vertex attribute pointer.
///
/// A `View` is used for a single attribute of a set of data, such as
/// Position or Normal.
pub struct View<'a> {
    /// The `Data` that contains the actual vertex attribute data
    data: &'a Data<'a>,
    // Number of elements per vertex - 1 to 4
    count: gl::types::GLint,
    /// The type of each element, e.g. GL_FLOAT
    gl_type : gl::types::GLenum,
    /// Offset from start of buffer to first byte of data
    offset : usize,
    /// Stride of data in the buffer - 0 for count*sizeof(gl_type)
    stride : gl::types::GLint,
}

//ip View
impl<'a> View<'a> {
    //fp new
    /// Create a new view of a `Data`
    pub fn new(data:&'a Data<'a>, count:usize, gl_type:gl::types::GLenum, offset:usize, stride:isize) -> Self {
        let count = count as gl::types::GLint;
        let stride = stride as gl::types::GLint;
        Self { data, count, gl_type, offset, stride }
    }

    //mp gl_create
    /// Create the OpenGL buffer required by the View
    pub fn gl_create(&self) {
        self.data.gl_create()
    }

    //mp gl_bind_attribute
    /// Bind this view to a particular attribute
    pub fn gl_bind_attribute(&self, attr:gl::types::GLuint) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.data.gl_buffer());
            gl::EnableVertexAttribArray(attr);
            gl::VertexAttribPointer(attr, // index
                                    self.count, // size
                                    self.gl_type, // types
                                    gl::FALSE, // normalized
                                    self.stride, // stride
                                    std::mem::transmute::<usize, *const std::os::raw::c_void>(self.offset) // ptr
            );
        }
    }

    //zz All done
}
/*
    #f hier_debug
    def hier_debug(self, hier:Hierarchy, use:str) -> Hierarchy:
        hier.add(f"BufferView {use} {self.gl_type} {self.count} {self.offset} {self.stride}")
        return hier
*/
