use {Result, Value};

pub struct Buffer {
    name   : String,
    uri    : String,
    length : usize,
    data   : Vec<byte>,
}
impl Buffer {
    pub fn new(name:&str, uri:&str, length:usize) -> Self {
        let name = from(name);
        let uri  = from(uri);
        Self { name, uri, length,
               data   : Vec::new(),
        }
    }
    pub fn of_json(json:serde_json::Value) -> Self {
        let name = json.get("name").unwrap().as_str().unwrap();
        let uri  = json.get("uri").unwrap().as_str().unwrap();
        let length = (json.get("byteLength").unwrap().as_u64().unwrap()) as usize;
        Self::new(name, uri, length)
    }
    // pub fn populate
        // if self.uri[:17] == "data:application/":
        //    data = self.uri.split(";base64,")[1]
        //    self.data = np.frombuffer(base64.b64decode(data),dtype=np.uint8)
        //   pass
        // else:
        //    path = Path(self.uri)
        //    with path.open("rb") as f:
        //        data = f.read()
    //       self.data = np.frombuffer(data, dtype=np.uint8)
}

pub struct BufferView {
    name   : String,
    buffer : usize,
    offset : usize,
    length : usize,
    stride : usize,
    // target : int
}

impl BufferView {
    pub fn new(name:&str, buffer:usize, offset:usize, length:usize, stride:usize) -> Self {
        let name = from(name);
        Self {name, buffer, offset, length, stride}
    }
    pub fn of_json(json:serde_json::Value) -> Self {
        let buffer = (json.get("buffer").unwrap().as_u64().unwrap()) as usize;
        let name = json.get("name").unwrap().as_str().unwrap();
        let stride = (json.get("byteStride").unwrap().as_u64().unwrap()) as usize;
        let offset = (json.get("byteOffset").unwrap().as_u64().unwrap()) as usize;
        let length = (json.get("byteLength").unwrap().as_u64().unwrap()) as usize;
        Self::new(name, buffer, offset, length, stride )
    }
}

pub struct Accessor {
    name: String,
    // BufferView index
    view: usize,
    /// Number of Float, Vec3, etc
    count: usize
    /// n'th item at byte view.offset + this.offset + view.stride*n
    offset: int #
    /// Svale, Vec3, Mat4, etc
    acc_type: CompType
    /// Byte, short, int
    comp_type: ValueType
}
#c Accessor
class Accessor:
    #f __init__
    def __init__(self, gltf:"Gltf", json:Json) -> None:
        self.view = gltf.get_buffer_view(json["bufferView"])
        self.name   = json.get("name","")
        self.offset = json.get("byteOffset",0)
        self.acc_type  = cast(CompType, CompType.of_name(json.get("type","SCALAR")))
        self.comp_type = cast(ValueType, ValueType.of_enum(json.get("componentType",5120)))
        self.count = json.get("count",0)
        pass
    #f to_model_buffer_view
    def to_model_buffer_view(self) -> ModelBufferView:
        data        = self.view.buffer.data
        byte_offset = self.view.offset
        byte_length = self.view.length
        stride      = self.view.stride
        offset      = self.offset
        count       = self.acc_type.size # e.g. 3 for VEC3
        gl_type     = self.comp_type.gl_type # e.g. of GL_FLOAT
        # print(f"Creating attributes of {gl_type} {byte_offset}, {byte_length}, {data[byte_offset:byte_offset+byte_length]}")
        model_data  = ModelBufferData(data=data, byte_offset=byte_offset, byte_length=byte_length)
        return ModelBufferView(data=model_data, count=count, gl_type=gl_type, offset=offset, stride=stride)
    #f to_model_buffer_indices
    def to_model_buffer_indices(self) -> ModelBufferIndices:
        data        = self.view.buffer.data
        byte_offset = self.view.offset
        byte_length = self.view.length
        # print(f"Creating indices of {self.comp_type.gl_type} {byte_offset}, {byte_length}, {data[byte_offset:byte_offset+byte_length]}")
        return ModelBufferIndices(data=data, byte_offset=byte_offset, byte_length=byte_length)
    #f All done
    pass

