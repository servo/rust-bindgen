//! Generate Rust bindings for C and C++ libraries.
//!
//! Provide a C/C++ header file, receive Rust FFI code to call into C/C++
//! functions and use types defined in the header.
//!
//! See the [Builder](./struct.Builder.html) struct for usage.

#![deny(missing_docs)]
#![deny(warnings)]
#![deny(unused_extern_crates)]

// We internally use the deprecated BindgenOptions all over the place. Once we
// remove its `pub` declaration, we can un-deprecate it and remove this pragma.
#![allow(deprecated)]

// To avoid rather annoying warnings when matching with CXCursor_xxx as a
// constant.
#![allow(non_upper_case_globals)]

#[macro_use]
#[allow(unused_extern_crates)]
extern crate cfg_if;
extern crate cexpr;
extern crate syntex_syntax as syntax;
extern crate aster;
extern crate quasi;
extern crate clang_sys;
extern crate regex;
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "logging")]
#[macro_use]
extern crate log;

#[cfg(not(feature = "logging"))]
#[macro_use]
mod log_stubs;

// A macro to declare an internal module for which we *must* provide
// documentation for. If we are building with the "testing_only_docs" feature,
// then the module is declared public, and our `#![deny(missing_docs)]` pragma
// applies to it. This feature is used in CI, so we won't let anything slip by
// undocumented. Normal builds, however, will leave the module private, so that
// we don't expose internals to library consumers.
macro_rules! doc_mod {
    ($m:ident, $doc_mod_name:ident) => {
        cfg_if! {
            if #[cfg(feature = "testing_only_docs")] {
                pub mod $doc_mod_name {
                    //! Autogenerated documentation module.
                    pub use super::$m::*;
                }
            } else {
            }
        }
    };
}

mod clang;
mod ir;
mod parse;
mod regex_set;
mod uses;

pub mod callbacks;

#[cfg(rustfmt)]
mod codegen;

doc_mod!(clang, clang_docs);
doc_mod!(ir, ir_docs);
doc_mod!(parse, parse_docs);
doc_mod!(regex_set, regex_set_docs);
doc_mod!(uses, uses_docs);

mod codegen {
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

use ir::context::{BindgenContext, ItemId};
use ir::item::Item;
use parse::{ClangItemParser, ParseError};
use regex_set::RegexSet;

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::sync::Arc;

use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::print::pp::eof;
use syntax::print::pprust;
use syntax::ptr::P;

/// A type used to indicate which kind of items do we have to generate.
///
/// TODO(emilio): Use `bitflags!`
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Whether to generate functions.
    pub functions: bool,
    /// Whether to generate types.
    pub types: bool,
    /// Whether to generate constants.
    pub vars: bool,
    /// Whether to generate methods.
    pub methods: bool,
    /// Whether to generate constructors.
    pub constructors: bool,
    /// Whether to generate destructors.
    pub destructors: bool,
}

impl CodegenConfig {
    /// Generate all kinds of items.
    pub fn all() -> Self {
        CodegenConfig {
            functions: true,
            types: true,
            vars: true,
            methods: true,
            constructors: true,
            destructors: true,
        }
    }

    /// Generate nothing.
    pub fn nothing() -> Self {
        CodegenConfig {
            functions: false,
            types: false,
            vars: false,
            methods: false,
            constructors: false,
            destructors: false,
        }
    }
}

impl Default for CodegenConfig {
    fn default() -> Self {
        CodegenConfig::all()
    }
}

/// Configure and generate Rust bindings for a C/C++ header.
///
/// This is the main entry point to the library.
///
/// ```ignore
/// use bindgen::builder;
///
/// // Configure and generate bindings.
/// let bindings = try!(builder().header("path/to/input/header")
///                              .whitelisted_type("SomeCoolClass")
///                              .whitelisted_function("do_some_cool_thing")
///                              .generate());
///
/// // Write the generated bindings to an output file.
/// try!(bindings.write_to_file("path/to/output.rs"));
/// ```
#[derive(Debug,Default)]
pub struct Builder {
    options: BindgenOptions,
}

/// Construct a new [`Builder`](./struct.Builder.html).
pub fn builder() -> Builder {
    Default::default()
}

impl Builder {
    /// Set the input C/C++ header.
    pub fn header<T: Into<String>>(mut self, header: T) -> Builder {
        let header = header.into();
        self.options.input_header = Some(header);
        self
    }

    /// Add `contents` as an input C/C++ header named `name`.
    ///
    /// The file `name` will be added to the clang arguments.
    pub fn header_contents(mut self, name: &str, contents: &str) -> Builder {
        self.options.input_unsaved_files.push(clang::UnsavedFile::new(name, contents));
        self
    }

    /// Set the output graphviz file.
    pub fn emit_ir_graphviz<T: Into<String>>(mut self, path: T) -> Builder {
        let path = path.into();
        self.options.emit_ir_graphviz = Some(path);
        self
    }

    /// Whether the generated bindings should contain documentation comments or
    /// not.
    ///
    /// This ideally will always be true, but it may need to be false until we
    /// implement some processing on comments to work around issues as described
    /// in:
    ///
    /// https://github.com/servo/rust-bindgen/issues/426
    pub fn generate_comments(mut self, doit: bool) -> Self {
        self.options.generate_comments = doit;
        self
    }

    /// Whether to whitelist types recursively or not. Defaults to true.
    ///
    /// This can be used to get bindgen to generate _exactly_ the types you want
    /// in your bindings, and then import other types manually via other means
    /// (like `raw_line`).
    pub fn whitelist_recursively(mut self, doit: bool) -> Self {
        self.options.whitelist_recursively = doit;
        self
    }

    /// Generate '#[macro_use] extern crate objc;' instead of 'use objc;'
    /// in the prologue of the files generated from objective-c files
    pub fn objc_extern_crate(mut self, doit: bool) -> Self {
        self.options.objc_extern_crate = doit;
        self
    }

    /// Whether to use the clang-provided name mangling. This is true and
    /// probably needed for C++ features.
    ///
    /// However, some old libclang versions seem to return incorrect results in
    /// some cases for non-mangled functions, see [1], so we allow disabling it.
    ///
    /// [1]: https://github.com/servo/rust-bindgen/issues/528
    pub fn trust_clang_mangling(mut self, doit: bool) -> Self {
        self.options.enable_mangling = doit;
        self
    }

    /// Generate a C/C++ file that includes the header and has dummy uses of
    /// every type defined in the header.
    pub fn dummy_uses<T: Into<String>>(mut self, dummy_uses: T) -> Builder {
        self.options.dummy_uses = Some(dummy_uses.into());
        self
    }

    /// Hide the given type from the generated bindings. Regular expressions are
    /// supported.
    pub fn hide_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.options.hidden_types.insert(arg);
        self
    }

    /// Treat the given type as opaque in the generated bindings. Regular
    /// expressions are supported.
    pub fn opaque_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.options.opaque_types.insert(arg);
        self
    }

    /// Whitelist the given type so that it (and all types that it transitively
    /// refers to) appears in the generated bindings. Regular expressions are
    /// supported.
    pub fn whitelisted_type<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.options.whitelisted_types.insert(arg);
        self
    }

    /// Whitelist the given function so that it (and all types that it
    /// transitively refers to) appears in the generated bindings. Regular
    /// expressions are supported.
    pub fn whitelisted_function<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.options.whitelisted_functions.insert(arg);
        self
    }

    /// Whitelist the given variable so that it (and all types that it
    /// transitively refers to) appears in the generated bindings. Regular
    /// expressions are supported.
    pub fn whitelisted_var<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.options.whitelisted_vars.insert(arg);
        self
    }

    /// Mark the given enum (or set of enums, if using a pattern) as being
    /// bitfield-like. Regular expressions are supported.
    ///
    /// This makes bindgen generate a type that isn't a rust `enum`. Regular
    /// expressions are supported.
    pub fn bitfield_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.options.bitfield_enums.insert(arg);
        self
    }

    /// Mark the given enum (or set of enums, if using a pattern) as being
    /// constant.
    ///
    /// This makes bindgen generate constants instead of enums. Regular
    /// expressions are supported.
    pub fn constified_enum<T: AsRef<str>>(mut self, arg: T) -> Builder {
        self.options.constified_enums.insert(arg);
        self
    }

    /// Add a string to prepend to the generated bindings. The string is passed
    /// through without any modification.
    pub fn raw_line<T: Into<String>>(mut self, arg: T) -> Builder {
        self.options.raw_lines.push(arg.into());
        self
    }

    /// Add an argument to be passed straight through to clang.
    pub fn clang_arg<T: Into<String>>(mut self, arg: T) -> Builder {
        self.options.clang_args.push(arg.into());
        self
    }

    /// Make the generated bindings link the given shared library.
    pub fn link<T: Into<String>>(mut self, library: T) -> Builder {
        self.options.links.push((library.into(), LinkType::Default));
        self
    }

    /// Make the generated bindings link the given static library.
    pub fn link_static<T: Into<String>>(mut self, library: T) -> Builder {
        self.options.links.push((library.into(), LinkType::Static));
        self
    }

    /// Make the generated bindings link the given framework.
    pub fn link_framework<T: Into<String>>(mut self, library: T) -> Builder {
        self.options.links.push((library.into(), LinkType::Framework));
        self
    }

    /// Emit bindings for builtin definitions (for example `__builtin_va_list`)
    /// in the generated Rust.
    pub fn emit_builtins(mut self) -> Builder {
        self.options.builtins = true;
        self
    }

    /// Avoid converting floats to f32/f64 by default.
    pub fn no_convert_floats(mut self) -> Self {
        self.options.convert_floats = false;
        self
    }

    /// Set whether `Debug` should be derived by default.
    pub fn derive_debug(mut self, doit: bool) -> Self {
        self.options.derive_debug = doit;
        self
    }

    /// Set whether `Default` should be derived by default.
    pub fn derive_default(mut self, doit: bool) -> Self {
        self.options.derive_default = doit;
        self
    }

    /// Emit Clang AST.
    pub fn emit_clang_ast(mut self) -> Builder {
        self.options.emit_ast = true;
        self
    }

    /// Emit IR.
    pub fn emit_ir(mut self) -> Builder {
        self.options.emit_ir = true;
        self
    }

    /// Enable C++ namespaces.
    pub fn enable_cxx_namespaces(mut self) -> Builder {
        self.options.enable_cxx_namespaces = true;
        self
    }

    /// Disable auto-namespacing of names if namespaces are disabled.
    ///
    /// By default, if namespaces are disabled, bindgen tries to mangle the
    /// names to from `foo::bar::Baz` to look like `foo_bar_Baz`, instead of
    /// just `Baz`.
    ///
    /// This option disables that behavior.
    ///
    /// Note that this intentionally doesn't change the names using for
    /// whitelisting and blacklisting, that should still be mangled with the
    /// namespaces.
    ///
    /// Note, also, that using this option may cause duplicated names to be
    /// generated.
    pub fn disable_name_namespacing(mut self) -> Builder {
        self.options.disable_name_namespacing = true;
        self
    }

    /// Treat inline namespaces conservatively.
    ///
    /// This is tricky, because in C++ is technically legal to override an item
    /// defined in an inline namespace:
    ///
    /// ```cpp
    /// inline namespace foo {
    ///     using Bar = int;
    /// }
    /// using Bar = long;
    /// ```
    ///
    /// Even though referencing `Bar` is a compiler error.
    ///
    /// We want to support this (arguably esoteric) use case, but we don't want
    /// to make the rest of bindgen users pay an usability penalty for that.
    ///
    /// To support this, we need to keep all the inline namespaces around, but
    /// then bindgen usage is a bit more difficult, because you cannot
    /// reference, e.g., `std::string` (you'd need to use the proper inline
    /// namespace).
    ///
    /// We could complicate a lot of the logic to detect name collisions, and if
    /// not detected generate a `pub use inline_ns::*` or something like that.
    ///
    /// That's probably something we can do if we see this option is needed in a
    /// lot of cases, to improve it's usability, but my guess is that this is
    /// not going to be too useful.
    pub fn conservative_inline_namespaces(mut self) -> Builder {
        self.options.conservative_inline_namespaces = true;
        self
    }

    /// Whether inline functions should be generated or not.
    ///
    /// Note that they will usually not work. However you can use
    /// `-fkeep-inline-functions` or `-fno-inline-functions` if you are
    /// responsible of compiling the library to make them callable.
    pub fn generate_inline_functions(mut self, doit: bool) -> Self {
        self.options.generate_inline_functions = doit;
        self
    }

    /// Ignore functions.
    pub fn ignore_functions(mut self) -> Builder {
        self.options.codegen_config.functions = false;
        self
    }

    /// Ignore methods.
    pub fn ignore_methods(mut self) -> Builder {
        self.options.codegen_config.methods = false;
        self
    }

    /// Avoid generating any unstable Rust, such as Rust unions, in the generated bindings.
    pub fn no_unstable_rust(mut self) -> Builder {
        self.options.unstable_rust = false;
        self
    }

    /// Use core instead of libstd in the generated bindings.
    pub fn use_core(mut self) -> Builder {
        self.options.use_core = true;
        self
    }

    /// Use the given prefix for the raw types instead of `::std::os::raw`.
    pub fn ctypes_prefix<T: Into<String>>(mut self, prefix: T) -> Builder {
        self.options.ctypes_prefix = Some(prefix.into());
        self
    }

    /// Allows configuring types in different situations, see the `ParseCallbacks`
    /// documentation.
    pub fn parse_callbacks(mut self, cb: Box<callbacks::ParseCallbacks>) -> Self {
        self.options.parse_callbacks = Some(cb);
        self
    }

    /// Choose what to generate using a CodegenConfig.
    pub fn with_codegen_config(mut self, config: CodegenConfig) -> Self {
        self.options.codegen_config = config;
        self
    }

    /// Prepend the enum name to constant or bitfield variants.
    pub fn prepend_enum_name(mut self, doit: bool) -> Self {
        self.options.prepend_enum_name = doit;
        self
    }

    /// Generate the Rust bindings using the options built up thus far.
    pub fn generate<'ctx>(self) -> Result<Bindings<'ctx>, ()> {
        Bindings::generate(self.options, None)
    }
}

/// Configuration options for generated bindings.
///
/// Deprecated: use a `Builder` instead.
#[derive(Debug)]
#[deprecated]
pub struct BindgenOptions {
    /// The set of types that have been blacklisted and should not appear
    /// anywhere in the generated code.
    pub hidden_types: RegexSet,

    /// The set of types that should be treated as opaque structures in the
    /// generated code.
    pub opaque_types: RegexSet,

    /// The set of types that we should have bindings for in the generated
    /// code.
    ///
    /// This includes all types transitively reachable from any type in this
    /// set. One might think of whitelisted types/vars/functions as GC roots,
    /// and the generated Rust code as including everything that gets marked.
    pub whitelisted_types: RegexSet,

    /// Whitelisted functions. See docs for `whitelisted_types` for more.
    pub whitelisted_functions: RegexSet,

    /// Whitelisted variables. See docs for `whitelisted_types` for more.
    pub whitelisted_vars: RegexSet,

    /// The enum patterns to mark an enum as bitfield.
    pub bitfield_enums: RegexSet,

    /// The enum patterns to mark an enum as constant.
    pub constified_enums: RegexSet,

    /// Whether we should generate builtins or not.
    pub builtins: bool,

    /// The set of libraries we should link in the generated Rust code.
    pub links: Vec<(String, LinkType)>,

    /// True if we should dump the Clang AST for debugging purposes.
    pub emit_ast: bool,

    /// True if we should dump our internal IR for debugging purposes.
    pub emit_ir: bool,

    /// Output graphviz dot file.
    pub emit_ir_graphviz: Option<String>,

    /// True if we should emulate C++ namespaces with Rust modules in the
    /// generated bindings.
    pub enable_cxx_namespaces: bool,

    /// True if we should avoid mangling names with namespaces.
    pub disable_name_namespacing: bool,

    /// True if we shold derive Debug trait implementations for C/C++ structures
    /// and types.
    pub derive_debug: bool,

    /// True if we shold derive Default trait implementations for C/C++ structures
    /// and types.
    pub derive_default: bool,

    /// True if we can use unstable Rust code in the bindings, false if we
    /// cannot.
    pub unstable_rust: bool,

    /// True if we should avoid using libstd to use libcore instead.
    pub use_core: bool,

    /// An optional prefix for the "raw" types, like `c_int`, `c_void`...
    pub ctypes_prefix: Option<String>,

    /// True if we should generate constant names that are **directly** under
    /// namespaces.
    pub namespaced_constants: bool,

    /// True if we should use MSVC name mangling rules.
    pub msvc_mangling: bool,

    /// Whether we should convert float types to f32/f64 types.
    pub convert_floats: bool,

    /// The set of raw lines to prepend to the generated Rust code.
    pub raw_lines: Vec<String>,

    /// The set of arguments to pass straight through to Clang.
    pub clang_args: Vec<String>,

    /// The input header file.
    pub input_header: Option<String>,

    /// Unsaved files for input.
    pub input_unsaved_files: Vec<clang::UnsavedFile>,

    /// Generate a dummy C/C++ file that includes the header and has dummy uses
    /// of all types defined therein. See the `uses` module for more.
    pub dummy_uses: Option<String>,

    /// A user-provided visitor to allow customizing different kinds of
    /// situations.
    pub parse_callbacks: Option<Box<callbacks::ParseCallbacks>>,

    /// Which kind of items should we generate? By default, we'll generate all
    /// of them.
    pub codegen_config: CodegenConfig,

    /// Whether to treat inline namespaces conservatively.
    ///
    /// See the builder method description for more details.
    pub conservative_inline_namespaces: bool,

    /// Wether to keep documentation comments in the generated output. See the
    /// documentation for more details.
    pub generate_comments: bool,

    /// Whether to generate inline functions. Defaults to false.
    pub generate_inline_functions: bool,

    /// Wether to whitelist types recursively. Defaults to true.
    pub whitelist_recursively: bool,

    /// Intead of emitting 'use objc;' to files generated from objective c files,
    /// generate '#[macro_use] extern crate objc;'
    pub objc_extern_crate: bool,

    /// Whether to use the clang-provided name mangling. This is true and
    /// probably needed for C++ features.
    ///
    /// However, some old libclang versions seem to return incorrect results in
    /// some cases for non-mangled functions, see [1], so we allow disabling it.
    ///
    /// [1]: https://github.com/servo/rust-bindgen/issues/528
    pub enable_mangling: bool,

    /// Whether to prepend the enum name to bitfield or constant variants.
    pub prepend_enum_name: bool,
}

/// TODO(emilio): This is sort of a lie (see the error message that results from
/// removing this), but since we don't share references across panic boundaries
/// it's ok.
impl ::std::panic::UnwindSafe for BindgenOptions {}

impl BindgenOptions {
    fn build(&mut self) {
        self.whitelisted_vars.build();
        self.whitelisted_types.build();
        self.whitelisted_functions.build();
        self.hidden_types.build();
        self.opaque_types.build();
        self.bitfield_enums.build();
        self.constified_enums.build();
    }
}

impl Default for BindgenOptions {
    fn default() -> BindgenOptions {
        BindgenOptions {
            hidden_types: Default::default(),
            opaque_types: Default::default(),
            whitelisted_types: Default::default(),
            whitelisted_functions: Default::default(),
            whitelisted_vars: Default::default(),
            bitfield_enums: Default::default(),
            constified_enums: Default::default(),
            builtins: false,
            links: vec![],
            emit_ast: false,
            emit_ir: false,
            emit_ir_graphviz: None,
            derive_debug: true,
            derive_default: false,
            enable_cxx_namespaces: false,
            disable_name_namespacing: false,
            unstable_rust: true,
            use_core: false,
            ctypes_prefix: None,
            namespaced_constants: true,
            msvc_mangling: false,
            convert_floats: true,
            raw_lines: vec![],
            clang_args: vec![],
            input_header: None,
            input_unsaved_files: vec![],
            dummy_uses: None,
            parse_callbacks: None,
            codegen_config: CodegenConfig::all(),
            conservative_inline_namespaces: false,
            generate_comments: true,
            generate_inline_functions: false,
            whitelist_recursively: true,
            objc_extern_crate: false,
            enable_mangling: true,
            prepend_enum_name: true,
        }
    }
}

/// The linking type to use with a given library.
///
/// TODO: #104: This is ignored at the moment, but shouldn't be.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkType {
    /// Use shared library linking. This is the default.
    Default,
    /// Use static linking.
    Static,
    /// The library is an OSX framework.
    Framework,
}

fn ensure_libclang_is_loaded() {
    if clang_sys::is_loaded() {
        return;
    }

    // XXX (issue #350): Ensure that our dynamically loaded `libclang`
    // doesn't get dropped prematurely, nor is loaded multiple times
    // across different threads.

    lazy_static! {
        static ref LIBCLANG: Arc<clang_sys::SharedLibrary> = {
            clang_sys::load().expect("Unable to find libclang");
            clang_sys::get_library()
                .expect("We just loaded libclang and it had better still be \
                        here!")
        };
    }

    clang_sys::set_library(Some(LIBCLANG.clone()));
}

/// Generated Rust bindings.
#[derive(Debug)]
pub struct Bindings<'ctx> {
    context: BindgenContext<'ctx>,
    module: ast::Mod,
}

impl<'ctx> Bindings<'ctx> {
    /// Generate bindings for the given options.
    ///
    /// Deprecated - use a `Builder` instead
    #[deprecated]
    pub fn generate(mut options: BindgenOptions,
                    span: Option<Span>)
                    -> Result<Bindings<'ctx>, ()> {
        let span = span.unwrap_or(DUMMY_SP);
        ensure_libclang_is_loaded();

        options.build();

        // TODO: Make this path fixup configurable?
        if let Some(clang) = clang_sys::support::Clang::find(None) {
            // If --target is specified, assume caller knows what they're doing
            // and don't mess with include paths for them
            let has_target_arg = options.clang_args
                .iter()
                .rposition(|arg| arg.starts_with("--target"))
                .is_some();
            if !has_target_arg {
                // TODO: distinguish C and C++ paths? C++'s should be enough, I
                // guess.
                for path in clang.cpp_search_paths.into_iter() {
                    if let Ok(path) = path.into_os_string().into_string() {
                        options.clang_args.push("-isystem".to_owned());
                        options.clang_args.push(path);
                    }
                }
            }
        }

        if let Some(h) = options.input_header.as_ref() {
            options.clang_args.push(h.clone())
        }

        for f in options.input_unsaved_files.iter() {
            options.clang_args.push(f.name.to_str().unwrap().to_owned())
        }

        let mut context = BindgenContext::new(options);
        try!(parse(&mut context));

        let module = ast::Mod {
            inner: span,
            items: codegen::codegen(&mut context),
        };

        Ok(Bindings {
            context: context,
            module: module,
        })
    }

    /// Convert these bindings into a Rust AST.
    pub fn into_ast(self) -> Vec<P<ast::Item>> {
        self.module.items
    }

    /// Convert these bindings into source text (with raw lines prepended).
    pub fn to_string(&self) -> String {
        let mut mod_str = vec![];
        {
            let ref_writer = Box::new(mod_str.by_ref()) as Box<Write>;
            self.write(ref_writer).expect("Could not write bindings to string");
        }
        String::from_utf8(mod_str).unwrap()
    }

    /// Write these bindings as source text to a file.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = try!(OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path));
        self.write(Box::new(file))
    }

    /// Write these bindings as source text to the given `Write`able.
    pub fn write<'a>(&self, mut writer: Box<Write + 'a>) -> io::Result<()> {
        try!(writer.write("/* automatically generated by rust-bindgen */\n\n"
            .as_bytes()));

        for line in self.context.options().raw_lines.iter() {
            try!(writer.write(line.as_bytes()));
            try!(writer.write("\n".as_bytes()));
        }
        if !self.context.options().raw_lines.is_empty() {
            try!(writer.write("\n".as_bytes()));
        }

        let mut ps = pprust::rust_printer(writer);
        try!(ps.print_mod(&self.module, &[]));
        try!(ps.print_remaining_comments());
        try!(eof(&mut ps.s));
        ps.s.out.flush()
    }

    /// Generate and write dummy uses of all the types we parsed, if we've been
    /// requested to do so in the options.
    ///
    /// See the `uses` module for more information.
    pub fn write_dummy_uses(&mut self) -> io::Result<()> {
        let file = if let Some(ref dummy_path) =
            self.context.options().dummy_uses {
            Some(try!(OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(dummy_path)))
        } else {
            None
        };

        if let Some(file) = file {
            try!(uses::generate_dummy_uses(&mut self.context, file));
        }

        Ok(())
    }
}

/// Determines whether the given cursor is in any of the files matched by the
/// options.
fn filter_builtins(ctx: &BindgenContext, cursor: &clang::Cursor) -> bool {
    ctx.options().builtins || !cursor.is_builtin()
}

/// Parse one `Item` from the Clang cursor.
pub fn parse_one(ctx: &mut BindgenContext,
                 cursor: clang::Cursor,
                 parent: Option<ItemId>)
                 -> clang_sys::CXChildVisitResult {
    if !filter_builtins(ctx, &cursor) {
        return CXChildVisit_Continue;
    }

    use clang_sys::CXChildVisit_Continue;
    match Item::parse(cursor, parent, ctx) {
        Ok(..) => {}
        Err(ParseError::Continue) => {}
        Err(ParseError::Recurse) => {
            cursor.visit(|child| parse_one(ctx, child, parent));
        }
    }
    CXChildVisit_Continue
}

/// Parse the Clang AST into our `Item` internal representation.
fn parse(context: &mut BindgenContext) -> Result<(), ()> {
    use clang_sys::*;

    let mut any_error = false;
    for d in context.translation_unit().diags().iter() {
        let msg = d.format();
        let is_err = d.severity() >= CXDiagnostic_Error;
        println!("{}, err: {}", msg, is_err);
        any_error |= is_err;
    }

    if any_error {
        return Err(());
    }

    let cursor = context.translation_unit().cursor();

    if context.options().emit_ast {

        fn dump_if_not_builtin(cur: &clang::Cursor) -> CXChildVisitResult {
            if !cur.is_builtin() {
                clang::ast_dump(&cur, 0)
            } else {
                CXChildVisit_Continue
            }
        }
        cursor.visit(|cur| dump_if_not_builtin(&cur));
    }

    let root = context.root_module();
    context.with_module(root, |context| {
        cursor.visit(|cursor| parse_one(context, cursor, None))
    });

    assert!(context.current_module() == context.root_module(),
            "How did this happen?");
    Ok(())
}

/// Extracted Clang version data
#[derive(Debug)]
pub struct ClangVersion {
    /// Major and minor semvar, if parsing was successful
    pub parsed: Option<(u32, u32)>,
    /// full version string
    pub full: String,
}

/// Get the major and the minor semvar numbers of Clang's version
pub fn clang_version() -> ClangVersion {
    if !clang_sys::is_loaded() {
        // TODO(emilio): Return meaningful error (breaking).
        clang_sys::load().expect("Unable to find libclang");
    }

    let raw_v: String = clang::extract_clang_version();
    let split_v: Option<Vec<&str>> = raw_v.split_whitespace()
        .nth(2)
        .map(|v| v.split('.').collect());
    match split_v {
        Some(v) => {
            if v.len() >= 2 {
                let maybe_major = v[0].parse::<u32>();
                let maybe_minor = v[1].parse::<u32>();
                match (maybe_major, maybe_minor) {
                    (Ok(major), Ok(minor)) => {
                        return ClangVersion {
                            parsed: Some((major, minor)),
                            full: raw_v.clone(),
                        }
                    }
                    _ => {}
                }
            }
        }
        None => {}
    };
    ClangVersion {
        parsed: None,
        full: raw_v.clone(),
    }
}
