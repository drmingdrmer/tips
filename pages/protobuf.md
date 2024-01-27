- **JSON Mapping**
  https://protobuf.dev/programming-guides/proto3/#json
- **prost_types** Protocol Buffers well-known types.
  https://protobuf.dev/reference/protobuf/google.protobuf/
  https://docs.rs/prost-types/latest/prost_types/index.html
-
- **Text Format Language Specification**
  https://protobuf.dev/reference/protobuf/textformat-spec/
	- use `prost_reflect` to convert to and from text fromat, see below
	  https://docs.rs/prost-reflect/latest/prost_reflect/struct.DynamicMessage.html#method.parse_text_format
-
- #### Language Specification
  https://protobuf.com/docs/language-spec
- #### Standard Imports
  https://protobuf.com/docs/descriptors#standard-imports
	- struct:
	  https://github.com/protocolbuffers/protobuf/blob/v21.3/src/google/protobuf/struct.proto
-
-
-
-
- Protobuf reflect
	- **protofish** parse proto **source**, dynamic parse
	  https://docs.rs/protofish/0.5.2/protofish/index.html
	- **prost_reflect** load **descriptor**, dynamic parse
	  https://docs.rs/prost-reflect/latest/prost_reflect/