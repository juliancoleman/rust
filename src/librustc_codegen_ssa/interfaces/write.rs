// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use back::lto::{LtoModuleCodegen, SerializedModule, ThinModule};
use back::write::{CodegenContext, ModuleConfig};
use {CompiledModule, ModuleCodegen};

use rustc::dep_graph::WorkProduct;
use rustc::util::time_graph::Timeline;
use rustc_errors::{FatalError, Handler};

pub trait WriteBackendMethods: 'static + Sized + Clone {
    type Module: Send + Sync;
    type TargetMachine: Clone;
    type ModuleBuffer: ModuleBufferMethods;
    type Context: ?Sized;
    type ThinData: Send + Sync;
    type ThinBuffer: ThinBufferMethods;

    /// Performs LTO, which in the case of full LTO means merging all modules into
    /// a single one and returning it for further optimizing. For ThinLTO, it will
    /// do the global analysis necessary and return two lists, one of the modules
    /// the need optimization and another for modules that can simply be copied over
    /// from the incr. comp. cache.
    fn run_lto(
        cgcx: &CodegenContext<Self>,
        modules: Vec<ModuleCodegen<Self::Module>>,
        cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
        timeline: &mut Timeline,
    ) -> Result<(Vec<LtoModuleCodegen<Self>>, Vec<WorkProduct>), FatalError>;
    fn print_pass_timings(&self);
    unsafe fn optimize(
        cgcx: &CodegenContext<Self>,
        diag_handler: &Handler,
        module: &ModuleCodegen<Self::Module>,
        config: &ModuleConfig,
        timeline: &mut Timeline,
    ) -> Result<(), FatalError>;
    unsafe fn optimize_thin(
        cgcx: &CodegenContext<Self>,
        thin: &mut ThinModule<Self>,
        timeline: &mut Timeline,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError>;
    unsafe fn codegen(
        cgcx: &CodegenContext<Self>,
        diag_handler: &Handler,
        module: ModuleCodegen<Self::Module>,
        config: &ModuleConfig,
        timeline: &mut Timeline,
    ) -> Result<CompiledModule, FatalError>;
    fn run_lto_pass_manager(
        cgcx: &CodegenContext<Self>,
        llmod: &ModuleCodegen<Self::Module>,
        config: &ModuleConfig,
        thin: bool,
    );
}

pub trait ThinBufferMethods: Send + Sync {
    fn data(&self) -> &[u8];
}

pub trait ModuleBufferMethods: Send + Sync {
    fn data(&self) -> &[u8];
}
