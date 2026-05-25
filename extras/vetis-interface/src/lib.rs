use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use serde::Deserialize;

use vetis::{Request, Response, errors::{ConfigError, VetisError}, virtual_host::path::Path};
#[cfg(feature = "asgi")]
use vetis_asgi::AsgiWorker;
#[cfg(feature = "fcgi")]
use vetis_fcgi::FcgiWorker;
#[cfg(feature = "rack")]
use vetis_rack::RackWorker;
#[cfg(feature = "rsgi")]
use vetis_rsgi::RsgiWorker;
#[cfg(feature = "sapi")]
use vetis_sapi::SapiWorker;
#[cfg(feature = "scgi")]
use vetis_scgi::ScgiWorker;
#[cfg(feature = "wsgi")]
use vetis_wsgi::WsgiWorker;

/// Interface enum
pub enum Interface {
    /// ASGI interface
    #[cfg(feature = "asgi")]
    Asgi(AsgiWorker),
    /// FCGI interface
    #[cfg(feature = "fcgi")]
    Fcgi(FcgiWorker),
    /// RACK interface
    #[cfg(feature = "rack")]
    Rack(RackWorker),
    /// RSGI interface
    #[cfg(feature = "rsgi")]
    Rsgi(RsgiWorker),
    /// SAPI interface
    #[cfg(feature = "sapi")]
    Sapi(SapiWorker),
    /// SCGI interface
    #[cfg(feature = "scgi")]
    Scgi(ScgiWorker),
    /// WSGI interface
    #[cfg(feature = "wsgi")]
    Wsgi(WsgiWorker),
}

/// Interface worker trait
pub trait InterfaceWorker {
    /// Handle the request
    fn handle(
        &self,
        request: Arc<Request>,
        uri: Arc<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'static>>;
}

/// Interface type for the path.
///
/// This enum defines the different interface types that can be used for a path.
///
/// # Variants
///
/// * `Asgi` - ASGI interface type for Python.
/// * `Wsgi` - WSGI interface type for Python.
/// * `Rsgi` - RSGI interface type for Python.
/// * `Sapi` - SAPI interface type for PHP.
/// * `Fcgi` - FCGI interface type.
/// * `Scgi` - SCGI interface type.
/// * `Rack` - Rack interface type for Ruby.
#[derive(Clone, Deserialize)]
#[non_exhaustive]
pub enum InterfaceType {
    /// ASGI interface type for Python.
    Asgi,
    /// WSGI interface type for Python.
    Wsgi,
    /// RSGI interface type for Python.
    Rsgi,
    /// SAPI interface type for PHP.
    Sapi,
    /// FCGI interface type.
    Fcgi,
    /// SCGI interface type.
    Scgi,
    /// Rack interface type for Ruby.
    Rack,
}

/// Builder for creating `InterfacePathConfig` instances.
pub struct InterfacePathConfigBuilder {
    uri: String,
    directory: String,
    target: String,
    params: Option<HashMap<String, String>>,
    interface_type: InterfaceType,
}

impl InterfacePathConfigBuilder {
    /// Allow set the URI of the static path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = uri.to_string();
        self
    }

    /// Allow set the directory of the interface path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn directory(mut self, directory: &str) -> Self {
        self.directory = directory.to_string();
        self
    }

    /// Allow set the target of the interface path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn target(mut self, target: &str) -> Self {
        self.target = target.to_string();
        self
    }

    /// Allow set the params of the interface path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn params(mut self, params: HashMap<String, String>) -> Self {
        self.params = Some(params);
        self
    }

    /// Allow set the interface type of the interface path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn interface_type(mut self, interface_type: InterfaceType) -> Self {
        self.interface_type = interface_type;
        self
    }

    /// Build the `InterfacePathConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `Result<InterfacePathConfig, VetisError>` - The `InterfacePathConfig` with the configured settings.
    pub fn build(self) -> Result<InterfacePathConfig, VetisError> {
        if self.uri.is_empty() {
            return Err(VetisError::Config(ConfigError::Path("URI cannot be empty".to_string())));
        }

        if self
            .directory
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::Path("Missing directory".to_string())));
        } else {
            let path = std::path::Path::new(&self.directory);
            if !path.exists() {
                return Err(VetisError::Config(ConfigError::Path(
                    "Directory does not exist".to_string(),
                )));
            }
        }

        if self
            .target
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::Path("Missing target".to_string())));
        } else {
            match self.interface_type {
                InterfaceType::Asgi => {
                    unimplemented!("ASGI interface type is not implemented yet")
                }
                InterfaceType::Rsgi => {
                    unimplemented!("RSGI interface type is not implemented yet")
                }
                InterfaceType::Wsgi => {
                    let target_parts = self
                        .target
                        .split_once(":");
                    match target_parts {
                        Some((module, _application)) => {
                            let path = std::path::Path::new(&self.directory);
                            let file = path.join(format!("{}.py", module));
                            if !file.exists() {
                                return Err(VetisError::Config(ConfigError::Path(
                                    "Module file does not exist".to_string(),
                                )));
                            }
                        }
                        None => {
                            return Err(VetisError::Config(ConfigError::Path("Target must be in format 'module:application' for API interface type".to_string())));
                        }
                    }
                }
                InterfaceType::Sapi => {
                    unimplemented!("SAPI interface type is not implemented yet")
                }
                InterfaceType::Fcgi => {
                    unimplemented!("FCGI interface type is not implemented yet")
                }
                InterfaceType::Scgi => {
                    unimplemented!("SCGI interface type is not implemented yet")
                }
                InterfaceType::Rack => {
                    unimplemented!("Rack interface type is not implemented yet")
                }
                _ => {
                    return Err(VetisError::Config(ConfigError::Path(
                        "Unsupported interface type".to_string(),
                    )));
                }
            }
        }

        Ok(InterfacePathConfig {
            uri: self.uri,
            directory: self.directory,
            target: self.target,
            params: self.params,
            interface_type: self.interface_type,
        })
    }
}

/// Interface path configuration.
#[derive(Clone, Deserialize)]
pub struct InterfacePathConfig {
    uri: String,
    directory: String,
    target: String,
    params: Option<HashMap<String, String>>,
    interface_type: InterfaceType,
}

impl InterfacePathConfig {
    /// Allow create a new `InterfacePathConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `InterfacePathConfigBuilder` - The builder.
    pub fn builder() -> InterfacePathConfigBuilder {
        InterfacePathConfigBuilder {
            uri: "/".to_string(),
            directory: ".".to_string(),
            target: "main".to_string(),
            params: None,
            interface_type: InterfaceType::Wsgi,
        }
    }

    /// Returns uri
    ///
    /// # Returns
    ///
    /// * `&str` - The uri.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns directory
    ///
    /// # Returns
    ///
    /// * `&str` - The directory.
    pub fn directory(&self) -> &str {
        &self.directory
    }

    /// Returns target
    ///
    /// # Returns
    ///
    /// * `&str` - The target.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns params
    ///
    /// # Returns
    ///
    /// * `&Option<HashMap<String, String>>` - The params.
    pub fn params(&self) -> &Option<HashMap<String, String>> {
        &self.params
    }

    /// Returns interface type
    ///
    /// # Returns
    ///
    /// * `&InterfaceType` - The interface type.
    pub fn interface_type(&self) -> &InterfaceType {
        &self.interface_type
    }
}

#[cfg(feature = "asgi")]
impl From<AsgiWorker> for Interface {
    /// Convert static path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The static path to convert
    ///
    /// # Returns
    ///
    /// * `Interface` - The interface
    fn from(value: AsgiWorker) -> Self {
        Interface::Asgi(value)
    }
}

#[cfg(feature = "fcgi")]
impl From<FcgiWorker> for Interface {
    /// Convert static path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The static path to convert
    ///
    /// # Returns
    ///
    /// * `Interface` - The interface
    fn from(value: FcgiWorker) -> Self {
        Interface::Fcgi(value)
    }
}

#[cfg(feature = "scgi")]
impl From<ScgiWorker> for Interface {
    /// Convert static path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The static path to convert
    ///
    /// # Returns
    ///
    /// * `Interface` - The interface
    fn from(value: ScgiWorker) -> Self {
        Interface::Scgi(value)
    }
}

#[cfg(feature = "sapi")]
impl From<SapiWorker> for Interface {
    /// Convert static path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The static path to convert
    ///
    /// # Returns
    ///
    /// * `Interface` - The interface
    fn from(value: SapiWorker) -> Self {
        Interface::Sapi(value)
    }
}

#[cfg(feature = "rack")]
impl From<RackWorker> for Interface {
    /// Convert static path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The static path to convert
    ///
    /// # Returns
    ///
    /// * `Interface` - The interface
    fn from(value: RackWorker) -> Self {
        Interface::Rack(value)
    }
}

#[cfg(feature = "rsgi")]
impl From<RsgiWorker> for Interface {
    /// Convert static path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The static path to convert
    ///
    /// # Returns
    ///
    /// * `Interface` - The interface
    fn from(value: RsgiWorker) -> Self {
        Interface::Rsgi(value)
    }
}

#[cfg(feature = "wsgi")]
impl From<WsgiWorker> for Interface {
    /// Convert static path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The static path to convert
    ///
    /// # Returns
    ///
    /// * `Interface` - The interface
    fn from(value: WsgiWorker) -> Self {
        Interface::Wsgi(value)
    }
}

impl InterfaceWorker for Interface {
    /// Handles the request for the path
    ///
    /// # Arguments
    ///
    /// * `request` - The request to handle
    /// * `uri` - The URI of the path
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + '_>>` - The future that will handle the request
    fn handle(
        &self,
        request: Arc<Request>,
        uri: Arc<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'static>> {
        match self {
            #[cfg(feature = "asgi")]
            Interface::Asgi(handler) => handler.handle(request, uri),
            #[cfg(feature = "fcgi")]
            Interface::Fcgi(handler) => handler.handle(request, uri),
            #[cfg(feature = "rack")]
            Interface::Rack(handler) => handler.handle(request, uri),
            #[cfg(feature = "rsgi")]
            Interface::Rsgi(handler) => handler.handle(request, uri),
            #[cfg(feature = "sapi")]
            Interface::Sapi(handler) => handler.handle(request, uri),
            #[cfg(feature = "scgi")]
            Interface::Scgi(handler) => handler.handle(request, uri),
            #[cfg(feature = "wsgi")]
            Interface::Wsgi(handler) => handler.handle(request, uri),
            _ => {
                panic!("Unsupported interface type");
            }
        }
    }
}

/// Proxy path
pub struct InterfacePath {
    config: InterfacePathConfig,
    interface: Interface,
}

impl InterfacePath {
    /// Create a new proxy path with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The proxy path configuration
    ///
    /// # Returns
    ///
    /// * `InterfacePath` - The proxy path
    pub fn new(config: InterfacePathConfig) -> InterfacePath {
        let directory = config
            .directory()
            .to_string();
        let target = config
            .target()
            .to_string();

        let interface = match config.interface_type() {
            #[cfg(feature = "asgi")]
            InterfaceType::Asgi => Interface::Asgi(AsgiWorker::new(directory, target)),
            #[cfg(feature = "fcgi")]
            InterfaceType::Fcgi => {
                let worker = FcgiWorker::new(directory, target);
                match worker {
                    Ok(worker) => Interface::Fcgi(worker),
                    Err(e) => {
                        panic!("Could not initialize fcgi worker: {}", e);
                    }
                }
            }
            #[cfg(feature = "rack")]
            InterfaceType::Rack => Interface::Rack(RackWorker::new(directory, target)),
            #[cfg(feature = "rsgi")]
            InterfaceType::Rsgi => Interface::Rsgi(RsgiWorker::new(directory, target)),
            #[cfg(feature = "sapi")]
            InterfaceType::Sapi => Interface::Sapi(SapiWorker::new(directory, target)),
            #[cfg(feature = "scgi")]
            InterfaceType::Scgi => Interface::Scgi(ScgiWorker::new(directory, target)),
            #[cfg(feature = "wsgi")]
            InterfaceType::Wsgi => {
                let worker = WsgiWorker::new(directory, target);
                match worker {
                    Ok(worker) => Interface::Wsgi(worker),
                    Err(e) => {
                        panic!("Could not initialize wsgi worker: {}", e);
                    }
                }
            }
            _ => {
                panic!("Unsupported interface type");
            }
        };

        InterfacePath { config, interface }
    }
}

impl From<InterfacePath> for HostPath {
    /// Convert proxy path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The proxy path to convert
    ///
    /// # Returns
    ///
    /// * `HostPath` - The host path
    fn from(value: InterfacePath) -> Self {
        HostPath::Interface(value)
    }
}

impl Path for InterfacePath {
    /// Get the URI of the proxy path
    ///
    /// # Returns
    ///
    /// * `&str` - The URI of the proxy path
    fn uri(&self) -> &str {
        self.config.uri()
    }

    /// Handle proxy request
    ///
    /// # Arguments
    ///
    /// * `request` - The request to handle
    /// * `uri` - The URI of the request
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + '_>>` - The future that will resolve to the response
    fn handle(
        &self,
        request: Request,
        uri: Arc<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + Sync + '_>> {
        let request = Arc::new(request);

        Box::pin(async move {
            let response = match self
                .config
                .interface_type()
            {
                #[cfg(feature = "asgi")]
                InterfaceType::Asgi => self
                    .interface
                    .handle(request.clone(), uri),
                #[cfg(feature = "fcgi")]
                InterfaceType::Fcgi => self
                    .interface
                    .handle(request.clone(), uri),
                #[cfg(feature = "rack")]
                InterfaceType::Rack => self
                    .interface
                    .handle(request.clone(), uri),
                #[cfg(feature = "rsgi")]
                InterfaceType::Rsgi => self
                    .interface
                    .handle(request.clone(), uri),
                #[cfg(feature = "sapi")]
                InterfaceType::Sapi => self
                    .interface
                    .handle(request.clone(), uri),
                #[cfg(feature = "scgi")]
                InterfaceType::Scgi => self
                    .interface
                    .handle(request.clone(), uri),
                #[cfg(feature = "wsgi")]
                InterfaceType::Wsgi => self
                    .interface
                    .handle(request.clone(), uri),
                _ => {
                    panic!("Unsupported interface type");
                }
            };

            let response = response.await?;

            Ok::<Response, VetisError>(response)
        })
    }
}
