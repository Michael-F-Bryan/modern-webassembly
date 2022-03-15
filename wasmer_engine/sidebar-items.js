initSidebarItems({"constant":[["VERSION","Version number of this crate."]],"enum":[["DeserializeError","The Deserialize error can occur when loading a compiled Module from a binary."],["Export","The value of an export passed from one instance to another."],["ImportError","An ImportError."],["InstantiationError","An error while instantiating a module."],["LinkError","The WebAssembly.LinkError object indicates an error during module instantiation (besides traps from the start function)."],["SerializeError","The Serialize error can occur when serializing a compiled Module into a binary."]],"fn":[["is_wasm_pc","Returns whether the `pc`, according to globally registered information, is a wasm trap or not."],["register_frame_info","Registers a new compiled module’s frame information."],["resolve_imports","This function allows to match all imports of a `ModuleInfo` with concrete definitions provided by a `Resolver`."]],"struct":[["EngineId","A unique identifier for an Engine."],["ExportFunction","A function export value with an extra function pointer to initialize host environments."],["ExportFunctionMetadata","Extra metadata about `ExportFunction`s."],["FRAME_INFO","This is a global cache of backtrace frame information for all active"],["FrameInfo","Description of a frame in a backtrace for a `RuntimeError::trace`."],["FunctionExtent","Represents a continuous region of executable memory starting with a function entry point."],["GlobalFrameInfoRegistration","An RAII structure used to unregister a module’s frame information when the module is destroyed."],["MetadataHeader","Metadata header which holds an ABI version and the length of the remaining metadata."],["NamedResolverChain","A [`Resolver`] that links two resolvers together in a chain."],["NullResolver","`Resolver` implementation that always resolves to `None`. Equivalent to `()`."],["RuntimeError","A struct representing an aborted instruction execution, with a message indicating the cause."]],"trait":[["Artifact","An `Artifact` is the product that the `Engine` implementation produce and use."],["ChainableNamedResolver","A trait for chaining resolvers together."],["Engine","A unimplemented Wasmer `Engine`."],["NamedResolver","Import resolver connects imports with available exported values."],["Resolver","Import resolver connects imports with available exported values."],["Tunables","An engine delegates the creation of memories, tables, and globals to a foreign implementor of this trait."]]});