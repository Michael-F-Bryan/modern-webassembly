use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::{
    fornjot_v1::{FornjotV1, LogLevel},
    model_v1::{Metadata, ModelV1},
};
use anyhow::{Context as _, Error};
use structopt::StructOpt;
use tracing_subscriber::EnvFilter;
use wasmer::{ImportObject, Module, Store, WasmerEnv};

wit_bindgen_wasmer::import!("model-v1.wit");
wit_bindgen_wasmer::export!("fornjot-v1.wit");

fn main() -> Result<(), Error> {
    let env =
        EnvFilter::from_default_env().add_directive("host=debug".parse()?);
    tracing_subscriber::fmt().with_env_filter(env).init();

    let Args {
        model_dir,
        model,
        args,
    } = Args::from_args();

    let args: HashMap<_, _> =
        args.into_iter().map(|a| (a.key, a.value)).collect();

    tracing::info!(model_dir = %model_dir.display(), %model, ?args, "Starting");

    let models = load_models(&model_dir)?;

    for model in &models {
        let meta = model.metadata()?;

        tracing::debug!(?meta, "Loaded metadata");

        model.arguments.lock().unwrap().extend(args.clone());

        let shape = model
            .instance
            .generate()?
            .map_err(|e| Error::msg(e.message))?;
        tracing::debug!(?shape, "Generated model");
    }

    Ok(())
}

#[tracing::instrument(skip(model_dir))]
fn load_models(model_dir: &Path) -> Result<Vec<Model>, Error> {
    tracing::info!(model_dir = %model_dir.display(), "Loading models");

    let mut models = Vec::new();

    let entries = model_dir.read_dir().with_context(|| {
        format!("Unable to read the \"{}\" directory", model_dir.display())
    })?;

    for entry in entries {
        let path = match entry {
            Ok(e) => e.path(),
            Err(_) => continue,
        };
        if path.extension() != Some(OsStr::new("wasm")) {
            continue;
        }

        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let _span = tracing::info_span!("Loading model file", name);

        tracing::debug!(
            filename = %path.display(),
            "Reading the WebAssembly into memory"
        );
        let wasm = std::fs::read(&path)?;

        tracing::debug!("Loading the WebAssembly module");
        let store = Store::default();
        let module = Module::new(&store, &wasm)?;

        let arguments = Arc::new(Mutex::new(HashMap::new()));

        let mut imports = ImportObject::new();
        crate::fornjot_v1::add_to_imports(
            &store,
            &mut imports,
            Fornjot {
                arguments: Arc::clone(&arguments),
            },
        );

        tracing::debug!("Instantiating");
        let (instance, _) =
            ModelV1::instantiate(&store, &module, &mut imports)?;
        models.push(Model {
            instance,
            arguments,
        });
    }

    Ok(models)
}

type Arguments = Arc<Mutex<HashMap<String, String>>>;

struct Model {
    instance: ModelV1,
    arguments: Arguments,
}

impl Model {
    fn metadata(&self) -> Result<Metadata, Error> {
        self.instance.on_load().map_err(Error::from)
    }
}

#[derive(Clone, Debug, WasmerEnv)]
struct Fornjot {
    arguments: Arguments,
}

impl FornjotV1 for Fornjot {
    type Context = Arguments;

    fn log(&mut self, level: LogLevel, msg: &str) {
        todo!("{:?} {}", level, msg);
    }

    fn context_current(&mut self) -> Self::Context {
        Arc::clone(&self.arguments)
    }

    fn context_get_argument(
        &mut self,
        ctx: &Self::Context,
        name: &str,
    ) -> Option<String> {
        ctx.lock().unwrap().get(name).cloned()
    }
}

#[derive(Debug, StructOpt)]
struct Args {
    /// The directory to load models from.
    #[structopt(long, short, env, default_value = ".")]
    model_dir: PathBuf,
    /// The model to load.
    model: String,
    /// Additional arguments to pass to the model.
    #[structopt(parse(try_from_str))]
    args: Vec<Argument>,
}

#[derive(Debug)]
struct Argument {
    key: String,
    value: String,
}

impl FromStr for Argument {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let (key, value) =
            s.split_once("=").context("Expected a key=value pair")?;

        Ok(Argument {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        })
    }
}
