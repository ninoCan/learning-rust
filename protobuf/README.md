# Protobuf

Protobuf is a IDL (Interface Definition Language) create at Google and released to public in 2008

## Structure

Protobufs define a `message` as a schema for serialization and deserialization of data, and allow
for code generation.

## Tooling

Once installed the `protobuf` package with a package manager of choice,
it is going to be possible to make use of `protoc` CLI tool to perform serde operation and 
automatic code generation

### Code Generation

The code in this folder has been generated out of the schema defined in the `./models` folder, via the following command:

```sh
protoc --rust_out=. -I=models models/*.proto --rust_opt="experimental-codegen=enabled,kernel=upb" 
```

### Serde operations

Using the `--encode` and `--decode` flags,
the content of the `data/course.txt` can be transformed into the binary version in `data/courses.bin`, and viceversa.
For this to work, one needs to provide the specific message in the command line as follows:

```sh
# encode
cat data/courses.txt | protoc -imodels --encode=myfirstproto.mooc.course models/course.proto > data/course.bin

# decode
cat data/courses.bin | protoc -Imodels --decode=myfirstproto.mooc.Course models/course.proto
```

Or whether the schema is not known and one wants to simply deserialize a binary format without a schema, this can be achieved as such:

```sh
# decode any
cat data/course.bin | protoc --decode_raw 
```
